mod delete;
pub mod factory;
mod get;
mod get_active;
mod insert;
mod valid_creation;
mod valid_transition;

use self::factory::{Factory, Logic};
use super::repo::db::DbRepo;
use crate::services::state::StateServiceLogic;
use std::sync::Arc;

pub fn new(repo: Arc<dyn DbRepo>, state_logic: Arc<StateServiceLogic>) -> Arc<dyn Logic> {
    Arc::new(Factory { repo, state_logic })
}
