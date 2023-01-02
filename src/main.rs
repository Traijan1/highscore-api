#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::status::{self};
use rocket::serde::json::Json;

mod db;
mod score_entry;
use db::Db;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection, Database};
use score_entry::ScoreEntry;

#[get("/")]
async fn index() -> &'static str {
    "Hello World"
}

#[post("/score", data = "<score>")]
async fn post_score(mut db: Connection<Db>, score: Json<ScoreEntry>) -> (Status, Json<ScoreEntry>) {
    let result = sqlx::query(&format!(
        "INSERT INTO scores (name, score, project_id) VALUES ('{}', {}, 1)",
        score.name, score.score
    ))
    .execute(&mut *db)
    .await;

    if result.is_ok() {
        (Status::Created, score)
    } else {
        (Status::NotAcceptable, score)
    }
}

#[get("/score?<offset>&<limit>")]
async fn get_score(
    mut db: Connection<Db>,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Result<Json<Vec<ScoreEntry>>, status::BadRequest<String>> {
    let limit_string = match limit {
        Some(value) => format!("LIMIT {}", value),
        _ => String::new(),
    };

    let offset_string = match offset {
        Some(value) => format!("OFFSET {}", value),
        _ => String::new(),
    };

    let entries: Vec<_> = sqlx::query(&format!(
        "SELECT * from scores ORDER BY score DESC {} {}",
        limit_string, offset_string
    ))
    .fetch_all(&mut *db)
    .await
    .unwrap()
    .iter()
    .map(|row| ScoreEntry {
        name: row.try_get("name").unwrap(),
        score: row.try_get("score").unwrap(),
    })
    .collect();

    // TODO: Fehler einbauen wenn offset zu groß ist

    Ok(Json::from(entries))
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/api", routes![index, post_score, get_score])
}
