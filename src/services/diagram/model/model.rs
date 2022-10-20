use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone))]
pub struct Diagram {
    pub code: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub flows: HashMap<String, FlowModel>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(test, derive(Clone))]
pub struct FlowModel {
    pub is_initial_state: bool,
    pub transitions: Option<Vec<String>>,
}

#[cfg(test)]
impl PartialEq for Diagram {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
            && self.description == other.description
            && self.is_active == other.is_active
            && self.flows == other.flows
    }
}

#[cfg(test)]
impl PartialEq for FlowModel {
    fn eq(&self, other: &Self) -> bool {
        self.is_initial_state == other.is_initial_state && self.transitions == other.transitions
    }
}
