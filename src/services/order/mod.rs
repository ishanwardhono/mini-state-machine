mod logic;
mod handler {
    pub mod http;
}
pub mod model {
    pub mod entity;
    pub mod model;
    pub mod request;
    pub mod response;
}
mod repo {
    pub mod db;
    mod db_query;
}

use self::{handler::http::register_handler, logic::factory::Logic};
use crate::cores::{database::DbPool, http::middleware::auth::Authority};
use actix_web::Scope;
use std::sync::Arc;

use super::{action::ActionServiceLogic, diagram::DiagramServiceLogic};

pub fn new(
    pool: Arc<DbPool>,
    diagram_logic: Arc<DiagramServiceLogic>,
    action_logic: Arc<ActionServiceLogic>,
) -> Service {
    Service {
        factory: logic::new(repo::db::new(pool), diagram_logic, action_logic),
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
