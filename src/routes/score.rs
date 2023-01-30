use crate::data::db::Db;
use crate::data::guards::rate_limit_guard::RateLimitGuard;
use crate::data::score_entry::ScoreEntry;
use rocket::http::Status;
use rocket::response::status::{self};
use rocket::serde::json::{serde_json, Json, Value};
use rocket::Route;
use rocket_db_pools::sqlx::sqlite::SqliteRow;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection};
use rocket_governor::RocketGovernor;

const MAX_LIMIT: u32 = 100;

#[post("/score", format = "json", data = "<score>")]
async fn post_score(
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
    mut db: Connection<Db>,
    mut score: Json<ScoreEntry>,
) -> (Status, Json<ScoreEntry>) {
    let custom_data = score.custom.clone().map(|value| value.to_string());

    let result =
        sqlx::query("INSERT INTO scores (name, score, custom, project_id) VALUES (?, ?, ?, 1)")
            .bind(score.name.clone())
            .bind(score.score)
            .bind(custom_data)
            .execute(&mut *db)
            .await;

    if result.is_ok() {
        score.id = result.unwrap().last_insert_rowid() as u32;
        (Status::Created, score)
    } else {
        println!("{}", result.err().unwrap());
        (Status::NotAcceptable, score)
    }
}

#[get("/scores?<offset>&<limit>")]
async fn get_scores(
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
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

    let entries: Vec<_> = sql
        .bind(limit)
        .fetch_all(&mut *db)
        .await
        .unwrap()
        .iter()
        .map(get_score_from_row)
        .collect();

    Json::from(entries)
}

#[get("/score?<id>")]
async fn get_score_by_id(
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
    mut db: Connection<Db>,
    id: u32,
) -> Result<Json<ScoreEntry>, status::BadRequest<()>> {
    let sql = sqlx::query("SELECT * from scores where id = ?");

    let entries: Vec<_> = sql
        .bind(id)
        .fetch_all(&mut *db)
        .await
        .unwrap()
        .iter()
        .map(get_score_from_row)
        .collect();

    Ok(Json::from(entries[0].clone()))
}

#[delete("/score?<id>")]
async fn delete_score(
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
    mut db: Connection<Db>,
    id: u32,
) -> Result<(), status::BadRequest<()>> {
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
async fn put_score(
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
    mut db: Connection<Db>,
    score: Json<ScoreEntry>,
) {
    let custom_data = score.custom.clone().map(|value| value.to_string());

    sqlx::query("UPDATE scores set name = ?, score = ?, custom = ? where id = ?")
        .bind(score.name.clone())
        .bind(score.score)
        .bind(custom_data)
        .bind(score.id)
        .execute(&mut *db)
        .await
        .unwrap();
}

fn get_score_from_row(row: &SqliteRow) -> ScoreEntry {
    ScoreEntry {
        name: row.try_get("name").unwrap(),
        score: row.try_get("score").unwrap(),
        id: row.try_get("id").unwrap(),
        custom: match row.try_get("custom") {
            Ok(value) => match serde_json::from_str(value) {
                Ok(value) => Some(value),
                Err(_) => None,
            },
            Err(_) => None,
        },
    }
}

pub(super) fn routes() -> Vec<Route> {
    routes![
        post_score,
        get_scores,
        delete_score,
        put_score,
        get_score_by_id
    ]
}
