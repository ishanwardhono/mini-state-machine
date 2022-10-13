use super::{
    business::factory::{Business, BusinessFactory},
    repo::db::DbRepoImpl,
};
use crate::cores::{database::pg::DbPool, env::Config};
use std::sync::Arc;

pub type AuthService = Arc<dyn Business>;

pub fn new(cfg: Arc<Config>, pool: Arc<DbPool>) -> AuthService {
    BusinessFactory::new(cfg, DbRepoImpl::new(pool))
}
