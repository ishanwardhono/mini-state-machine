use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StateCreateRequest {
    pub code: String,
    pub description: Option<String>,
    pub webhooks: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StateUpdateRequest {
    pub description: Option<String>,
    pub webhooks: Option<Vec<String>>,
}
