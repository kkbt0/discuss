#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod module;
mod routes;
mod schema;

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;

use rocket_sync_db_pools::database;
use routes::*;

#[database("sqlite_main")]
pub struct MainDbConn(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        // db
        .attach(MainDbConn::fairing())
        // url discuss
        .mount("/", routes![get_url_discussion])
        .mount(
            "/",
            routes![post_discussion, get_discussion, get_single_discussion,post_discussion_optional],
        )
        .mount("/", routes![get_discussion_many])
        .attach(CORS)
}

pub struct CORS;
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("Setting access control allow origin");
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
  
    }
}