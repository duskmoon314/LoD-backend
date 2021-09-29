#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

mod db;
mod models;
mod req;
mod routes;

use db::DB;
use routes::pastebin;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(DB::init())
        .attach(AdHoc::on_ignite("mount_pastebin", pastebin::init))
}
