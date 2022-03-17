use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{ get, post, response::status::Created, response::Debug};

use crate::module::{
    DBGetDiscussion, DBInsertDiscussion, FrontPostDiscussion, GetDiscussionAndSonNode,GetDiscussionRange,
};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::schema::discuss_main::{self, dsl::*, son_nodes};
use crate::MainDbConn;
use diesel::{insert_into, prelude::*, QueryDsl, RunQueryDsl};

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

// URL / -> %2F
// URL : -> %3A
// URL https://www.ftls.xyz/posts/rust-rocket3/ -> https%3A%2F%2Fwww.ftls.xyz%2Fposts%2Frust-rocket3%2F
#[get("/url/<in_url>")]
pub async fn get_url_discussion(in_url: String) -> Value {
    json!({"res": "Jump to a url discuss","from_url":in_url})
}

// post discussion content from front
#[post("/dispost", format = "json", data = "<front_post_discussion>")]
pub async fn post_discussion(
    db: MainDbConn,
    front_post_discussion: Json<FrontPostDiscussion>,
) -> Result<Created<Json<FrontPostDiscussion>>> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    //    dbg!(timestamp);// as_secs as_millis
    let father_node = front_post_discussion.father_node;
    let db_insert_discussion = DBInsertDiscussion {
        author: front_post_discussion.author.clone(),
        content: front_post_discussion.content.clone(),
        created_at: timestamp.to_string(),
        up: 0,
        down: 0,
        read_number: 0,
        reply_to: front_post_discussion.reply_to.clone(),
        sharded_number: 0,
        father_nodes: father_node,
        son_nodes: "".to_string(),
    };
    // 插入并获取子节点id -> this_id
    db.run(move |conn| {
        insert_into(discuss_main)
            .values(&db_insert_discussion)
            .execute(conn)
      //  discuss_main::table.select(id).order(id.desc()).first::<i32>(conn)
    }).await?;
    let this_id = db.run(move |conn| {
        discuss_main::table.select(id).order(id.desc()).first::<i32>(conn)
    }).await?;

    // 获取 父节点 子节点 列表
    let father_node_str: Json<DBGetDiscussion> = db
        .run(move |conn| {
            discuss_main::table
                .filter(discuss_main::id.eq(father_node))
                .first(conn)
        })
        .await
        .map(Json)
        .ok()
        .unwrap();
    let new_father_son_nodes_str = format!("{}{}{}", father_node_str.into_inner().son_nodes,this_id," ");
    // 更新
    let affected = db
        .run(move |conn| {
            diesel::update(discuss_main::table.filter(discuss_main::id.eq(father_node)))
                .set(son_nodes.eq(new_father_son_nodes_str))
                .execute(conn)
        })
        .await?;
    (affected == 1).then(|| ());


    Ok(Created::new("/").body(front_post_discussion))
}

// get by id and it's son_nodes
#[get("/discussion/<in_id>")]
pub async fn get_discussion(db: MainDbConn, in_id: i32) -> Option<Json<GetDiscussionAndSonNode>> {
    let main_dis: Option<Json<DBGetDiscussion>> = db
        .run(move |conn| {
            discuss_main::table
                .filter(discuss_main::id.eq(in_id))
                .first(conn)
        })
        .await
        .map(Json)
        .ok();

    let mut son_nodes_vec_contents: Vec<Json<DBGetDiscussion>> = Vec::new();
    let main_dis = main_dis.unwrap().into_inner();
    let son_nodes_string = main_dis.son_nodes.clone();
    let son_nodes_vec_str = son_nodes_string.trim_end().split(" ").collect::<Vec<_>>();
    let mut son_nodes_vec_i32 = Vec::new();
    for i in son_nodes_vec_str {
        son_nodes_vec_i32.push(i.parse::<i32>().unwrap());
    }
    for in_id in son_nodes_vec_i32 {
        son_nodes_vec_contents.push(
            db.run(move |conn| {
                discuss_main::table
                    .filter(discuss_main::id.eq(in_id))
                    .first(conn)
            })
            .await
            .map(Json)
            .ok()
            .unwrap(),
        )
    }
    let mut vev_tem = Vec::new();
    for i in son_nodes_vec_contents {
        vev_tem.push(i.into_inner());
    }
    Some(Json(GetDiscussionAndSonNode {
        main_dis,
        child_dis: vev_tem,
    }))
}

// get discussion by id
#[get("/dis_sign/<in_id>")]
pub async fn get_single_discussion(db: MainDbConn, in_id: i32) -> Option<Json<DBGetDiscussion>> {
    db.run(move |conn| {
        discuss_main::table
            .filter(discuss_main::id.eq(in_id))
            .first(conn)
    })
    .await
    .map(Json)
    .ok()
}

// 根据列表SQL eg: 1,2,3 -> 1 2 3 
#[get("/dis_many/<in_str>")]
pub async fn get_discussion_many(db: MainDbConn,in_str: String) -> Option<Json<GetDiscussionRange>> {
    let in_str = in_str.replace(","," ");
    let mut son_nodes_vec_contents: Vec<Json<DBGetDiscussion>> = Vec::new();
    let son_nodes_vec_str = in_str.trim().split(" ").collect::<Vec<_>>();
    let mut son_nodes_vec_i32 = Vec::new();
    for i in son_nodes_vec_str {
        son_nodes_vec_i32.push(i.parse::<i32>().unwrap());
    }
    for in_id in son_nodes_vec_i32 {
        son_nodes_vec_contents.push(
            db.run(move |conn| {
                discuss_main::table
                    .filter(discuss_main::id.eq(in_id))
                    .first(conn)
            })
            .await
            .map(Json)
            .ok()
            .unwrap(),
        )
    }
    let mut vev_tem = Vec::new();
    for i in son_nodes_vec_contents {
        vev_tem.push(i.into_inner());
    }
    Some(Json(GetDiscussionRange {
        dis_listy: vev_tem,
    }))
}