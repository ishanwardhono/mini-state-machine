use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub id: i32,
    pub code: String,
    pub description: Option<String>,
    pub webhooks: Option<Vec<String>>,
    pub created_at: chrono::NaiveDateTime,
}
