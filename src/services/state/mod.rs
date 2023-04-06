pub mod logic;
mod handler {
    pub mod http;
}
pub mod model {
    pub mod entity;
    pub mod request;
    pub mod response;
}
mod repo {
    pub mod db;
    mod db_query;
}

use self::{handler::http::register_handler, logic::factory::Logic};
use super::client::ClientServiceLogic;
use crate::cores::database::DbPool;
use crate::cores::http::middleware::auth::Authority;
use actix_web::Scope;
use std::sync::Arc;

pub type StateServiceLogic = dyn Logic;

pub fn new(pool: Arc<DbPool>, client_logic: Arc<ClientServiceLogic>) -> Service {
    Service {
        factory: logic::new(repo::db::new(pool), client_logic),
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
