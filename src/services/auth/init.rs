use super::{
    business::factory::{Business, BusinessFactory},
    repo::db::DbRepoImpl,
};
use crate::cores::database::pg::DbPool;
use std::sync::Arc;

pub struct AuthService {
    pub factory: Arc<dyn Business>,
}

impl AuthService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            factory: BusinessFactory::new(DbRepoImpl::new(pool)),
        }
    }
}
