use std::sync::Arc;

use actix_web::Scope;

use crate::cores::database::DbPool;

use self::{
    business::{Business, BusinessFactory},
    repo::Repo,
};

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
            factory: BusinessFactory::new(Repo::new(pool)),
        }
    }

    pub fn init_http_service(&self) -> Scope {
        handler::http::register_handler(self.factory.clone())
    }
}
