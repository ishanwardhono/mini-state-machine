mod handler;
pub mod logic;
pub mod model;
mod repo;

use self::{handler::http::register_handler, logic::factory::Logic};
use crate::{
    cores::{database::pg::DbPool, http::middleware::auth::Authority},
    services::state::logic::factory as state_factory,
};
use actix_web::Scope;
use std::sync::Arc;

pub fn new(pool: Arc<DbPool>, state_factory: Arc<dyn state_factory::Logic>) -> Service {
    Service {
        factory: logic::new(repo::new(pool), state_factory),
    }
}

pub struct Service {
    pub factory: Arc<dyn Logic>,
}

impl Service {
    pub fn init_http_service(&self, auth: Authority) -> Scope {
        register_handler(self.factory.clone(), auth)
    }
}
