use self::state::StateService;
use crate::cores::database::DbPool;
use actix_web::{
    web::{self},
    Scope,
};
use std::sync::Arc;

mod state;

//Http Handler Registration
pub fn provider(pool: Arc<DbPool>) -> Scope {
    let service = StateService::new(pool.clone());
    let service2 = StateService::new(pool.clone());

    web::scope("/app")
        .service(service.init_http_service())
        .service(web::scope("servc1").service(service.init_http_service()))
        .service(web::scope("nested").service(service2.init_http_service()))
}
