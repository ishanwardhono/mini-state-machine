use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Diagram {
    pub code: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub flows: HashMap<String, FlowModel>,
}

#[derive(Deserialize, Serialize)]
pub struct FlowModel {
    pub is_initial_state: bool,
    pub transitions: Option<Vec<String>>,
}
