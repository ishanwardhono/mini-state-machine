pub mod factory;
mod get;
mod insert;

use self::factory::{Logic, LogicFactory};
use super::repo::db::DbRepo;
use crate::services::diagram;
use std::sync::Arc;

pub fn new(
    repo: Arc<dyn DbRepo>,
    diagram_factory: Arc<dyn diagram::logic::factory::Logic>,
) -> Arc<dyn Logic> {
    Arc::new(LogicFactory {
        repo,
        diagram_factory,
    })
}
