mod logic;
pub mod model {
    pub mod entity;
    pub mod request;
}
mod repo {
    pub mod db;
    mod db_query;
}

use self::logic::factory::Logic;
use crate::cores::{database::DbPool, env::Config};
use std::sync::Arc;

pub type Service = Arc<dyn Logic>;

pub fn new(cfg: Arc<Config>, pool: Arc<DbPool>) -> Service {
    logic::new(cfg, repo::db::new(pool))
}
