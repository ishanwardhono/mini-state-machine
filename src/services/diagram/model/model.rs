use serde::Deserialize;

#[derive(Deserialize)]
pub struct Diagram {
    pub code: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub flows: Vec<FlowModel>,
}

#[derive(Deserialize)]
pub struct FlowModel {
    pub state: String,
    pub is_initial_state: bool,
    pub next_states: Option<Vec<String>>,
}
