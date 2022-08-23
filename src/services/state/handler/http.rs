use crate::services::state::{
    business::Business,
    model::{InsertResponse, StateRequest},
};
use actix_web::{
    error::ErrorInternalServerError,
    web::{self, get, post},
    Error, HttpResponse, Scope,
};
use std::sync::Arc;

pub fn register_handler(factory: Arc<dyn Business>) -> Scope {
    web::scope("/states")
        .route("/", get().to(get_all))
        .route("/", post().to(insert))
        .route("/{id}", get().to(get_by_id))
        .app_data(web::Data::from(factory))
}

async fn get_all(factory: web::Data<dyn Business>) -> Result<HttpResponse, Error> {
    let result = factory.get_all().await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

async fn get_by_id(
    factory: web::Data<dyn Business>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let result = factory.get_by_id(id).await;
    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}

async fn insert(
    factory: web::Data<dyn Business>,
    req: web::Json<StateRequest>,
) -> Result<HttpResponse, Error> {
    let result = factory.insert(req.into_inner()).await;

    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(InsertResponse { is_success: res })),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
