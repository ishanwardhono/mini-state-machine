use std::sync::Arc;

use actix_web::{
    web::{self},
    Scope,
};

use crate::cores::database::DbPool;

use self::state::StateService;

mod state;

//Http Handler Registration
pub fn http_register(pool: Arc<DbPool>) -> Scope {
    let service = Arc::new(StateService::new(pool));
    web::scope("/app")
        .service(service.clone().init_http_service())
        .service(web::scope("nested").service(service.clone().init_http_service()))
}
