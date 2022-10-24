mod logic;
pub mod model;
mod repo;

use self::logic::factory::Logic;
use crate::cores::{database::pg::DbPool, env::Config};
use std::sync::Arc;

pub type Service = Arc<dyn Logic>;

pub fn new(cfg: Arc<Config>, pool: Arc<DbPool>) -> Service {
    logic::new(cfg, repo::db::new(pool))
}
