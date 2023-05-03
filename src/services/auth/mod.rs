mod logic;
pub mod model {
    pub mod entity;
    pub mod request;
    pub mod response;
}
mod repo {
    pub mod db;
    mod db_query;
}
mod handler {
    pub mod http;
}

use actix_web::Scope;

use self::{handler::http::register_handler, logic::factory::Logic};
use crate::cores::{database::DbPool, env::Config, http::middleware::auth::Authority};
use std::sync::Arc;

pub type AuthorityService = dyn Logic;

pub struct Service {
    pub factory: Arc<AuthorityService>,
}

pub fn new(cfg: Arc<Config>, pool: Arc<DbPool>) -> Service {
    Service {
        factory: logic::new(cfg, repo::db::new(pool)),
    }
}

impl Service {
    pub fn init_http_service(&self, auth: Authority) -> Scope {
        register_handler(self.factory.clone(), auth)
    }
}
