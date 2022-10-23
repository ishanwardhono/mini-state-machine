use super::{
    handler::http::register_handler,
    logic::factory::{Logic, LogicFactory},
    repo::db,
};
use crate::{
    cores::{database::pg::DbPool, http::middleware::auth::Authority},
    services::diagram::logic::factory::{self as diagram_factory},
};
use actix_web::Scope;
use std::sync::Arc;

pub struct OrderService {
    pub factory: Arc<dyn Logic>,
}

impl OrderService {
    pub fn new(pool: Arc<DbPool>, diagram_factory: Arc<dyn diagram_factory::Logic>) -> Self {
        Self {
            factory: LogicFactory::new(db::new(pool), diagram_factory),
        }
    }

    pub fn init_http_service(&self, auth: Authority) -> Scope {
        register_handler(self.factory.clone(), auth)
    }
}
