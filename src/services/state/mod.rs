use self::{
    business::{Business, BusinessFactory},
    repo::db::DbRepoImpl,
};
use crate::cores::database::DbPool;
use actix_web::Scope;
use std::sync::Arc;

mod business;
mod handler;
mod model;
mod repo;

pub struct StateService {
    pub factory: Arc<dyn Business>,
}

impl StateService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            factory: BusinessFactory::new(DbRepoImpl::new(pool)),
        }
    }

    pub fn init_http_service(&self) -> Scope {
        handler::http::register_handler(self.factory.clone())
    }
}
