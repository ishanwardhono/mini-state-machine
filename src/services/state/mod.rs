use actix_web::Scope;

use crate::cores::database::DbPool;

use self::{business::BusinessFactory, repo::Repo};

mod model;
mod repo;
mod handler;
mod business;

#[derive(Clone)]
pub struct StateService {
    pub factory: BusinessFactory
}

impl StateService {
    pub fn new(pool: DbPool) -> Self {
        let repo = Repo::new(pool);
        let factory = BusinessFactory::new(repo);
        Self { factory }
    }

    //TODO:
    //Are you sureee????!!!!
    //self reference not pointer
    pub fn init_http_service(self) -> Scope {
        handler::http::register_handler(self.factory)
    }
}
