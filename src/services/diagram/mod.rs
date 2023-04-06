mod handler {
    pub mod http;
}
pub mod logic;
pub mod model {
    pub mod entity;
    pub mod model;
    pub mod reponse;
}
mod repo {
    pub mod db;
    mod db_query;
}

use self::{handler::http::register_handler, logic::factory::Logic};
use super::state::StateServiceLogic;
use crate::cores::{database::DbPool, http::middleware::auth::Authority};
use actix_web::Scope;
use std::sync::Arc;

pub type DiagramServiceLogic = dyn Logic;

pub fn new(pool: Arc<DbPool>, state_logic: Arc<StateServiceLogic>) -> Service {
    Service {
        factory: logic::new(repo::db::new(pool), state_logic),
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
