pub mod factory;
mod run;
mod send;

use self::factory::{Factory, Logic};
use crate::services::{client::ClientServiceLogic, state::StateServiceLogic};
use std::sync::Arc;

pub fn new(
    client_logic: Arc<ClientServiceLogic>,
    state_logic: Arc<StateServiceLogic>,
) -> Arc<dyn Logic> {
    Arc::new(Factory {
        client_logic,
        state_logic,
    })
}
