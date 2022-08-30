use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StateRequest {
    pub code: String,
    pub description: Option<String>,
    pub webhooks: Option<Vec<String>>,
}
