mod handler;
mod logic;
pub mod model;
mod repo;

use self::{handler::http::register_handler, logic::factory::Logic};
use crate::cores::{database::pg::DbPool, http::middleware::auth::Authority};
use actix_web::Scope;
use std::sync::Arc;

use super::diagram::DiagramServiceLogic;

pub fn new(pool: Arc<DbPool>, diagram_logic: Arc<DiagramServiceLogic>) -> OrderService {
    OrderService {
        factory: logic::new(repo::db::new(pool), diagram_logic),
    }
}

pub struct OrderService {
    pub factory: Arc<dyn Logic>,
}

impl OrderService {
    pub fn init_http_service(&self, auth: Authority) -> Scope {
        register_handler(self.factory.clone(), auth)
    }
}
