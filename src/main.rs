#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod module;
mod routes;
mod schema;

use rocket_sync_db_pools::database;
use routes::*;

#[database("sqlite_main")]
pub struct MainDbConn(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        // db
        .attach(MainDbConn::fairing())
        .mount("/", routes![index])
        // add api
        .mount("/", routes![get_article_list, get_article_by_id])
        .mount("/", routes![post_article, delete_article, put_article])
        // url discuss
        .mount("/", routes![get_url_discussion])
        .mount(
            "/",
            routes![post_discussion, get_discussion, get_single_discussion],
        )
        .mount("/", routes![get_discussion_many])
        .mount("/", routes![get_discussion_test])
}
