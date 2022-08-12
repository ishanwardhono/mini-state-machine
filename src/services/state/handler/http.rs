use actix_web::{web::{self, resource, get}, Error, Scope, HttpResponse};

use crate::services::state::business;

pub fn register_handler() -> Scope{
    web::scope("/states")
        .service(resource("/register").route(get().to(get_states))
    )
}

async fn get_states() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::BadGateway().finish()
    )
}