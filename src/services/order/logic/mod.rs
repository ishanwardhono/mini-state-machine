pub mod factory;
mod get;
mod insert;
mod state_update;
mod upsert;

use self::factory::{Factory, Logic};
use super::repo::db::DbRepo;
use crate::services::diagram::DiagramServiceLogic;
use std::sync::Arc;

pub fn new(repo: Arc<dyn DbRepo>, diagram_logic: Arc<DiagramServiceLogic>) -> Arc<dyn Logic> {
    Arc::new(Factory {
        repo,
        diagram_logic,
    })
}
