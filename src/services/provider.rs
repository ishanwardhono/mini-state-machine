use super::{action, auth, client, diagram, order, state};
use crate::cores::{
    database::pg::DbPool,
    env::Config,
    http::{self, middleware::auth::Authority},
};
use actix_web::{web, Scope};
use std::sync::Arc;

//Http Handler Registration
pub fn register(cfg: Arc<Config>, pool: Arc<DbPool>) -> Scope {
    let auth = get_authority(cfg, pool.clone());
    let clients = client::new(pool.clone());
    let states = state::new(pool.clone(), clients.factory.clone());
    let actions = action::new(clients.factory.clone(), states.factory.clone());
    let diagrams = diagram::new(pool.clone(), states.factory.clone());
    let orders = order::new(
        pool.clone(),
        diagrams.factory.clone(),
        actions.factory.clone(),
    );

    web::scope("/app")
        .service(clients.init_http_service(auth.clone()))
        .service(states.init_http_service(auth.clone()))
        .service(diagrams.init_http_service(auth.clone()))
        .service(orders.init_http_service(auth.clone()))
}

fn get_authority(cfg: Arc<Config>, pool: Arc<DbPool>) -> Authority {
    let service = auth::new(cfg, pool);
    http::middleware::auth::new(service)
}
