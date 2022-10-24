mod delete;
pub mod factory;
mod get;
mod get_active;
mod insert;
mod valid_creation;
mod valid_transition;

use self::factory::{Factory, Logic};
use super::repo::db::DbRepo;
use crate::services::state::logic::factory as StateFactory;
use std::sync::Arc;

pub fn new(repo: Arc<dyn DbRepo>, state_factory: Arc<dyn StateFactory::Logic>) -> Arc<dyn Logic> {
    Arc::new(Factory {
        repo,
        state_factory,
    })
}
