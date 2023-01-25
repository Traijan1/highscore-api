use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ScoreEntry {
    #[serde(default)]
    pub id: u32,
    pub name: String,
    pub score: u32,
}
