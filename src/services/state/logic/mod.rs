mod delete;
pub mod factory;
mod get_all;
mod get_by_code;
mod get_by_codes;
mod insert;
mod update;

use self::factory::{Factory, Logic};
use super::repo::db::DbRepo;
use std::sync::Arc;

pub fn new(repo: Arc<dyn DbRepo>) -> Arc<dyn Logic> {
    Arc::new(Factory { repo })
}
