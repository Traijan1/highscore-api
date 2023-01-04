#[macro_use]
extern crate rocket;

mod db;
mod routes;
mod score_entry;
mod tests;

use db::Db;
use rocket_db_pools::Database;
use routes::score::routes;

#[launch]
async fn rocket() -> _ {
    let routes = routes();

    rocket::build().attach(Db::init()).mount("/api", routes)
}
