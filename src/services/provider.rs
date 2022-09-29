use super::{auth as auth_service, state::init::StateService};
use crate::cores::{
    database::pg::DbPool,
    http::{self, middleware::auth::Authority},
};
use actix_web::{
    web::{self},
    Scope,
};
use std::sync::Arc;

//Http Handler Registration
pub fn register(pool: Arc<DbPool>) -> Scope {
    let service = StateService::new(pool.clone());
    let authority = new_authority(pool.clone());

    web::scope("/app").service(service.init_http_service(authority.clone()))
}

fn new_authority(pool: Arc<DbPool>) -> Authority {
    let service = auth_service::init::new(pool);
    http::middleware::auth::new(service)
}
