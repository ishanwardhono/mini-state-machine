use super::entity::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertResponse {
    pub state: State,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeResponse {
    pub code: String,
}
