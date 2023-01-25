#[macro_use]
extern crate rocket;

mod data;
mod routes;
mod tests;

use data::db::Db;
use rocket_db_pools::Database;
use routes::routes;

#[launch]
async fn rocket() -> _ {
    rocket::build().attach(Db::init()).mount("/api", routes())
}
