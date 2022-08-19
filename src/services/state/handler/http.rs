use crate::services::state::business::Business;
use actix_web::{
    error::ErrorInternalServerError,
    web::{self, get},
    Error, HttpResponse, Scope,
};
use std::sync::Arc;

pub fn register_handler(factory: Arc<dyn Business>) -> Scope {
    web::scope("/states")
        .route("/register", get().to(get_states))
        .route("/get-all", get().to(get_all))
        .app_data(web::Data::from(factory))
}

async fn get_states(factory: web::Data<dyn Business>) -> Result<HttpResponse, Error> {
    let result = factory.get_all().await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

async fn get_all(factory: web::Data<dyn Business>) -> Result<HttpResponse, Error> {
    let result = factory.get_all().await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
