use super::{
    logic::factory::{Logic, LogicFactory},
    repo::db::DbRepoImpl,
};
use crate::cores::{database::pg::DbPool, env::Config};
use std::sync::Arc;

pub type AuthService = Arc<dyn Logic>;

pub fn new(cfg: Arc<Config>, pool: Arc<DbPool>) -> AuthService {
    LogicFactory::new(cfg, DbRepoImpl::new(pool))
}
