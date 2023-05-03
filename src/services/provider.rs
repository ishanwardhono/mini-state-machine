use super::{action, auth, client, diagram, order, state};
use crate::cores::{database::DbPool, env::Config, http};
use actix_web::{web, Scope};
use std::sync::Arc;

//Http Handler Registration
pub fn register(cfg: Arc<Config>, pool: Arc<DbPool>) -> Scope {
    let auth_service = auth::new(cfg, pool.clone());
    let authority = http::middleware::auth::new(auth_service.factory.clone());
    let clients = client::new(pool.clone());
    let states = state::new(pool.clone(), clients.factory.clone());
    let actions = action::new(
        pool.clone(),
        clients.factory.clone(),
        states.factory.clone(),
    );
    let diagrams = diagram::new(pool.clone(), states.factory.clone());
    let orders = order::new(
        pool.clone(),
        diagrams.factory.clone(),
        actions.factory.clone(),
    );

    web::scope("/app")
        .service(auth_service.init_http_service(authority.clone()))
        .service(clients.init_http_service(authority.clone()))
        .service(states.init_http_service(authority.clone()))
        .service(diagrams.init_http_service(authority.clone()))
        .service(orders.init_http_service(authority.clone()))
}
