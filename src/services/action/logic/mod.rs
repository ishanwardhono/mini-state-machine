pub mod factory;
mod run;
mod send;

use self::factory::{Factory, Logic};
use crate::services::{client::ClientServiceLogic, state::StateServiceLogic};
use std::sync::Arc;

use super::repo::db::DbRepo;

pub fn new(
    repo: Arc<dyn DbRepo>,
    client_logic: Arc<ClientServiceLogic>,
    state_logic: Arc<StateServiceLogic>,
) -> Arc<dyn Logic> {
    Arc::new(Factory {
        repo,
        client_logic,
        state_logic,
    })
}
