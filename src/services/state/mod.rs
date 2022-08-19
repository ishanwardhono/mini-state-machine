use std::sync::Arc;

use actix_web::Scope;

use crate::cores::database::DbPool;

use self::{business::BusinessFactory, repo::Repo};

mod business;
mod handler;
mod model;
mod repo;

#[derive(Clone)]
pub struct StateService {
    pub factory: BusinessFactory,
}

impl StateService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        let repo = Repo::new(pool);
        let factory = BusinessFactory::new(repo);
        Self { factory }
    }

    //TODO:
    //check if field in Arc got cloned, is still working
    pub fn init_http_service(&self) -> Scope {
        handler::http::register_handler(self.factory.clone())
    }
}
