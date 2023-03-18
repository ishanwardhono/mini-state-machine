mod logic;
pub mod model;
mod repo;

use self::logic::factory::Logic;
use super::{client::ClientServiceLogic, state::StateServiceLogic};
use crate::cores::database::pg::DbPool;
use std::sync::Arc;

pub type ActionServiceLogic = dyn Logic;

pub fn new(
    pool: Arc<DbPool>,
    client_logic: Arc<ClientServiceLogic>,
    state_logic: Arc<StateServiceLogic>,
) -> Service {
    Service {
        factory: logic::new(
            repo::db::new(pool),
            client_logic.clone(),
            state_logic.clone(),
        ),
    }
}

pub struct Service {
    pub factory: Arc<dyn Logic>,
}
