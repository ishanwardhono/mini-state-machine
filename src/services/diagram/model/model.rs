use serde::Deserialize;

use super::entity::{Business, Flow};

#[derive(Deserialize)]
pub struct Diagram {
    pub business: Business,
    pub flows: Vec<Flow>,
}
