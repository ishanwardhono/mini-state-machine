use super::{
    handler::http::register_handler,
    logic::factory::{Logic, LogicFactory},
    repo::db::DbRepoImpl,
};
use crate::{
    cores::{database::pg::DbPool, http::middleware::auth::Authority},
    services::state::logic::factory as StateFactory,
};
use actix_web::Scope;
use std::sync::Arc;

pub struct DiagramService {
    pub factory: Arc<dyn Logic>,
}

impl DiagramService {
    pub fn new(pool: Arc<DbPool>, state_factory: Arc<dyn StateFactory::Logic>) -> Self {
        Self {
            factory: LogicFactory::new(DbRepoImpl::new(pool), state_factory),
        }
    }

    pub fn init_http_service(&self, auth: Authority) -> Scope {
        register_handler(self.factory.clone(), auth)
    }
}
