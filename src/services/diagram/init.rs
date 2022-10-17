use actix_web::Scope;

use crate::cores::http::middleware::auth::Authority;

use super::{
    business::factory::{Business, BusinessFactory},
    handler::http::register_handler,
};
use std::sync::Arc;

pub struct DiagramService {
    pub factory: Arc<dyn Business>,
}

impl DiagramService {
    pub fn new() -> Self {
        Self {
            factory: BusinessFactory::new(),
        }
    }

    pub fn init_http_service(&self, auth: Authority) -> Scope {
        register_handler(self.factory.clone(), auth)
    }
}
