use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ScoreEntry {
    pub name: String,
    pub score: u32,
}
