use rocket::{
    futures::executor::block_on, http::Status, local::asynchronous::Client, serde::json::Json,
};

use crate::data::score_entry::ScoreEntry;

use super::rocket;

#[async_test]
async fn test_post_score() {
    let rocket = rocket().await;
    let client = Client::tracked(rocket).await.unwrap();

    let response = client
        .post("/score")
        .body(
            r#"
            {
                "name": "Bob",
                "score": 0
            }
        "#,
        )
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Ok);
}

#[async_test]
async fn test_get_score() {
    let rocket = rocket().await;
    let client = Client::tracked(rocket).await.unwrap();
    let response = client.get("/score").dispatch().await;
    assert_eq!(response.status(), Status::Ok);

    let test: Vec<ScoreEntry> = response.into_json().await.unwrap();

    assert_eq!(test.len(), 5);
}
