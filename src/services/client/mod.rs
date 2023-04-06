pub mod logic;
mod repo {
    pub mod db;
    mod db_query;
}
mod handler {
    pub mod http;
}
mod model {
    pub mod entity;
    pub mod model;
    pub mod response;
}

use self::{handler::http::register_handler, logic::factory::Logic};
use crate::cores::database::DbPool;
use crate::cores::http::middleware::auth::Authority;
use actix_web::Scope;
use std::sync::Arc;

pub type ClientServiceLogic = dyn Logic;

pub fn new(pool: Arc<DbPool>) -> Service {
    Service {
        factory: logic::new(repo::db::new(pool)),
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
