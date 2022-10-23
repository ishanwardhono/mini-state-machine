use super::{
    auth as auth_service, diagram::init::DiagramService, order::init::OrderService,
    state::init::StateService,
};
use crate::cores::{
    database::pg::DbPool,
    env::Config,
    http::{self, middleware::auth::Authority},
};
use actix_web::{
    web::{self},
    Scope,
};
use std::sync::Arc;

//Http Handler Registration
pub fn register(cfg: Arc<Config>, pool: Arc<DbPool>) -> Scope {
    let auth = new_authority(cfg, pool.clone());
    let states = StateService::new(pool.clone());
    let diagrams = DiagramService::new(pool.clone(), states.factory.clone());
    let orders = OrderService::new(pool.clone(), diagrams.factory.clone());

    web::scope("/app")
        .service(states.init_http_service(auth.clone()))
        .service(diagrams.init_http_service(auth.clone()))
        .service(orders.init_http_service(auth.clone()))
}

fn new_authority(cfg: Arc<Config>, pool: Arc<DbPool>) -> Authority {
    let service = auth_service::init::new(cfg, pool);
    http::middleware::auth::new(service)
}
