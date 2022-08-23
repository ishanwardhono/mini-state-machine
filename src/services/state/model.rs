use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub id: i32,
    pub code: String,
    pub description: Option<String>,
    pub webhooks: Option<Vec<String>>,
    pub created_at: chrono::NaiveDateTime,
}

//Todo
//move this to dtos
#[derive(Debug, Serialize, Deserialize)]
pub struct StateRequest {
    pub code: String,
    pub description: Option<String>,
    pub webhooks: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertResponse {
    pub is_success: bool,
}
