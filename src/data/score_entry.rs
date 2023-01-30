use rocket::serde::{json::Value, Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ScoreEntry {
    #[serde(default)]
    pub id: u32,
    pub name: String,
    pub score: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<Value>,
}
