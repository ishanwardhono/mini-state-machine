use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StateCreateRequest {
    pub code: String,
    pub description: Option<String>,
    pub webhooks: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateUpdateRequest {
    pub description: Option<String>,
    pub webhooks: Option<Vec<String>>,
}
