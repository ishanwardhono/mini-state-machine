use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    pub id: i32,
    pub code: String,
    pub description: Option<String>,
    pub webhooks: Option<Vec<String>>,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
}
