pub mod factory;
mod get;
mod insert;
mod state_update;
mod upsert;

use self::factory::{Factory, Logic};
use super::repo::db::DbRepo;
use crate::services::{action::ActionServiceLogic, diagram::DiagramServiceLogic};
use std::sync::Arc;

pub fn new(
    repo: Arc<dyn DbRepo>,
    diagram_logic: Arc<DiagramServiceLogic>,
    action_logic: Arc<ActionServiceLogic>,
) -> Arc<dyn Logic> {
    Arc::new(Factory {
        repo,
        diagram_logic,
        action_logic,
    })
}
