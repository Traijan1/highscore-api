use crate::db::Db;
use crate::score_entry::ScoreEntry;
use rocket::http::Status;
use rocket::response::status::{self};
use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection};

const MAX_LIMIT: u32 = 100;

#[post("/score", format = "json", data = "<score>")]
async fn post_score(
    mut db: Connection<Db>,
    mut score: Json<ScoreEntry>,
) -> (Status, Json<ScoreEntry>) {
    println!("asdsa");
    let result = sqlx::query(&format!(
        "INSERT INTO scores (name, score, project_id) VALUES ('{}', {}, 1)",
        score.name, score.score
    ))
    .execute(&mut *db)
    .await;

    if result.is_ok() {
        score.id = result.unwrap().last_insert_rowid() as u32;
        (Status::Created, score)
    } else {
        (Status::NotAcceptable, score)
    }
}

#[get("/score?<offset>&<limit>")]
async fn get_score(
    mut db: Connection<Db>,
    offset: Option<usize>,
    limit: Option<u32>,
) -> Json<Vec<ScoreEntry>> {
    let limit = limit.unwrap_or(MAX_LIMIT).min(MAX_LIMIT);

    let (offset_string, offset) = match offset {
        Some(value) => (format!("WHERE id >= {}", value + 1), value as u32),
        _ => (String::new(), 0),
    };

    let sql_string = format!(
        "SELECT * from scores {} ORDER BY score DESC LIMIT ?",
        offset_string
    );

    let mut sql = sqlx::query(&sql_string);

    if !String::is_empty(&offset_string) {
        sql = sql.bind(offset);
    }

    println!("{}", sql_string);

    let entries: Vec<_> = sql
        .bind(limit)
        .fetch_all(&mut *db)
        .await
        .unwrap()
        .iter()
        .map(|row| ScoreEntry {
            name: row.try_get("name").unwrap(),
            score: row.try_get("score").unwrap(),
            id: row.try_get("id").unwrap(),
        })
        .collect();

    println!("{}", entries.len());

    Json::from(entries)
}

#[delete("/score?<id>")]
async fn delete_score(mut db: Connection<Db>, id: u32) -> Result<(), status::BadRequest<()>> {
    let result = sqlx::query("DELETE from scores where id = ?")
        .bind(id)
        .execute(&mut *db)
        .await;

    if result.is_ok() {
        Ok(())
    } else {
        Err(status::BadRequest(Some(())))
    }
}

#[put("/score", format = "json", data = "<score>")]
async fn put_score(mut db: Connection<Db>, score: Json<ScoreEntry>) {
    sqlx::query("UPDATE scores set name = ?, score = ? where id = ?")
        .bind(score.name.clone())
        .bind(score.score)
        .bind(score.id)
        .execute(&mut *db)
        .await
        .unwrap();
}

pub(super) fn routes() -> Vec<Route> {
    routes![post_score, get_score, delete_score, put_score]
}
