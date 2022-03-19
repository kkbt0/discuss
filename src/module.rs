use crate::schema::*;
use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

// get discussion and son_nodes struct
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GetDiscussionAndSonNode {
    pub main_dis: DBGetDiscussion,
    pub child_dis: Vec<DBGetDiscussion>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GetDiscussionRange {
    pub dis_listy: Vec<DBGetDiscussion>,
}

// father_node only one
// add father_node and add father node's son_nodes
// define when father_node == -1 ,discuss_mian is a root node

// post node info:
// author(f)
// content(f)
// created_at(b)
// up(b default 0)
// down(b default 0)
// read_number(b default 0)
// sharded_number(b default 0)
// father_node(f need judge)
// son_nodes(b default "NULL")
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct FrontPostDiscussion {
    pub author: String,
    pub content: String,
    pub father_node: i32,
    pub reply_to: i32,
}
#[derive(Serialize, Deserialize, Clone, Debug, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "discuss_main"]
pub struct DBInsertDiscussion {
    pub author: i32,
    pub content: String,
    pub created_at: String,
    pub up: i32,
    pub down: i32,
    pub read_number: i32,
    pub reply_to: i32,
    pub sharded_number: i32,
    pub father_nodes: i32,
    pub son_nodes: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct DBGetDiscussion {
    pub id: i32,
    pub author: i32,
    pub content: String,
    pub created_at: String,
    pub reply_to: i32,
    pub up: i32,
    pub down: i32,
    pub read_number: i32,
    pub sharded_number: i32,
    pub father_nodes: Option<i32>,
    pub son_nodes: String,
}

// nodes_list struct
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NodeList {
    pub id: i32,
    pub nodes_list: String,
}

// user_list Query From DB
#[derive(Serialize, Deserialize, Clone, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct UserKeyIDList {
    pub id: i32,           // to convent only sign
    pub key_id: String,    // user password
    pub show_name: String, // @xxx show
    pub status: i32,       // 权限等级
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserKeyIDListFrontUsed {
    pub id: i32,
    pub show_name: String,
    pub status: i32,
}
