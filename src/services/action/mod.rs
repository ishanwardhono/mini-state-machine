mod logic;
pub mod model;
mod repo;

use self::logic::factory::Logic;
use super::{client::ClientServiceLogic, state::StateServiceLogic};
use std::sync::Arc;

pub type ActionServiceLogic = dyn Logic;

pub fn new(client_logic: Arc<ClientServiceLogic>, state_logic: Arc<StateServiceLogic>) -> Service {
    Service {
        factory: logic::new(client_logic.clone(), state_logic.clone()),
    }
}

pub struct Service {
    pub factory: Arc<dyn Logic>,
}
