use rocket_db_pools::sqlx;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("scores")]
pub struct Db(sqlx::SqlitePool);
