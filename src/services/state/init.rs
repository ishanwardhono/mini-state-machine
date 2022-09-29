use super::business::factory::{Business, BusinessFactory};
use super::handler::http::register_handler;
use super::repo::db::DbRepoImpl;
use crate::cores::database::pg::DbPool;
use crate::services::auth::init::AuthService;
use actix_web::Scope;
use std::sync::Arc;

pub struct StateService {
    pub factory: Arc<dyn Business>,
}

impl StateService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            factory: BusinessFactory::new(DbRepoImpl::new(pool)),
        }
    }

    pub fn init_http_service(&self, auth: AuthService) -> Scope {
        register_handler(self.factory.clone(), auth)
    }
}
