use super::{
    auth::init::{new, AuthService},
    state::init::StateService,
};
use crate::cores::database::pg::DbPool;
use actix_web::{
    web::{self},
    Scope,
};
use std::sync::Arc;

//Http Handler Registration
pub fn register(pool: Arc<DbPool>) -> Scope {
    let service = StateService::new(pool.clone());
    let auth_service = new_auth_service(pool.clone());

    web::scope("/app").service(service.init_http_service(auth_service.clone()))
}

fn new_auth_service(pool: Arc<DbPool>) -> AuthService {
    new(pool)
}
