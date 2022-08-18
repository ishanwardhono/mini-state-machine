use actix_web::{web::{ServiceConfig, self}, Scope};

use crate::cores::database::DbPool;

use self::state::StateService;

mod state;

//Http Handler Registration
pub fn http_register(pool: DbPool) -> Scope {
    let service = StateService::new(pool);
    web::scope("/app")
        .service(service.init_http_service())
        // .service(service.init_http_service())
}