use actix_web::{web::{self, resource, get}, Error, Scope, HttpResponse, error::ErrorInternalServerError};

use crate::services::state::{business::BusinessFactory, model::State};

pub fn register_handler(factory: BusinessFactory) -> Scope {
    web::scope("/states")
        .route("/register", get().to(get_states))
        .route("/get-all", get().to(get_all))
        .app_data(web::Data::new(factory)
    )
}

async fn get_states(factory: web::Data<BusinessFactory>) -> Result<HttpResponse, Error> {
    let result = factory.get_all().await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Err(ErrorInternalServerError(e))
    }
}

async fn get_all(factory: web::Data<BusinessFactory>) -> Result<HttpResponse, Error> {
    let result = factory.get_all().await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Err(ErrorInternalServerError(e))
    }
}