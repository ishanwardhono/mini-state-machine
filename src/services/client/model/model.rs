use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClientModel {
    pub code: String,
    pub url: String,
}
