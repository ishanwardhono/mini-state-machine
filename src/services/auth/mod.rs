mod logic;
pub mod model;
mod repo;

use self::logic::factory::{self, Logic};
use crate::cores::{database::pg::DbPool, env::Config};
use std::sync::Arc;

pub type Service = Arc<dyn Logic>;

pub fn new(cfg: Arc<Config>, pool: Arc<DbPool>) -> Service {
    factory::new(cfg, repo::new(pool))
}
