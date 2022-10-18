use actix_web::Scope;

use crate::cores::http::middleware::auth::Authority;

use super::{
    handler::http::register_handler,
    logic::factory::{Logic, LogicFactory},
};
use std::sync::Arc;

pub struct DiagramService {
    pub factory: Arc<dyn Logic>,
}

impl DiagramService {
    pub fn new() -> Self {
        Self {
            factory: LogicFactory::new(),
        }
    }

    pub fn init_http_service(&self, auth: Authority) -> Scope {
        register_handler(self.factory.clone(), auth)
    }
}
