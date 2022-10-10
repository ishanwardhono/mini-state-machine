use super::{auth as auth_service, state::init::StateService};
use crate::cores::{
    database::pg::DbPool,
    environment::Config,
    http::{self, middleware::auth::Authority},
};
use actix_web::{
    web::{self},
    Scope,
};
use std::sync::Arc;

//Http Handler Registration
pub fn register(cfg: Arc<Config>, pool: Arc<DbPool>) -> Scope {
    let authority = new_authority(cfg, pool.clone());
    let service = StateService::new(pool.clone());

    web::scope("/app").service(service.init_http_service(authority.clone()))
}

fn new_authority(cfg: Arc<Config>, pool: Arc<DbPool>) -> Authority {
    let service = auth_service::init::new(cfg, pool);
    http::middleware::auth::new(service)
}
