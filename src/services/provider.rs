use crate::cores::database::pg::DbPool;
use actix_web::{
    web::{self},
    Scope,
};
use std::sync::Arc;

use super::state::init::StateService;

//Http Handler Registration
pub fn register(pool: Arc<DbPool>) -> Scope {
    let service = StateService::new(pool.clone());

    web::scope("/app").service(service.init_http_service())
}
