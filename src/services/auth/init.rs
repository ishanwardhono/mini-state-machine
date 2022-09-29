use super::{
    business::factory::{Business, BusinessFactory},
    repo::db::DbRepoImpl,
};
use crate::cores::database::pg::DbPool;
use std::sync::Arc;

pub type AuthService = Arc<dyn Business>;

pub fn new(pool: Arc<DbPool>) -> Arc<dyn Business> {
    BusinessFactory::new(DbRepoImpl::new(pool))
}
