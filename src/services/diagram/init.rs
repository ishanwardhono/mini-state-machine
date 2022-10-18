use actix_web::Scope;

use crate::cores::{database::pg::DbPool, http::middleware::auth::Authority};

use super::{
    handler::http::register_handler,
    logic::factory::{Logic, LogicFactory},
    repo::db::DbRepoImpl,
};
use std::sync::Arc;

pub struct DiagramService {
    pub factory: Arc<dyn Logic>,
}

impl DiagramService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            factory: LogicFactory::new(DbRepoImpl::new(pool)),
        }
    }

    pub fn init_http_service(&self, auth: Authority) -> Scope {
        register_handler(self.factory.clone(), auth)
    }
}
