use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Diagram {
    pub code: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub flows: Vec<FlowModel>,
}

#[derive(Deserialize, Serialize)]
pub struct FlowModel {
    pub state: String,
    pub is_initial_state: bool,
    pub next_states: Option<Vec<String>>,
}
