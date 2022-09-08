use crate::{
    cores::error::Error,
    services::state::{
        business::Business,
        model::{request::StateRequest, response::InsertResponse},
    },
};
use actix_web::{
    web::{self, get, post, put},
    HttpResponse, Scope,
};
use std::sync::Arc;

pub fn register_handler(factory: Arc<dyn Business>) -> Scope {
    web::scope("/states")
        .route("/", get().to(get_all))
        .route("/", post().to(insert))
        .route("/{id}", get().to(get_by_id))
        .route("/{id}", put().to(update))
        .route("/{id}", web::delete().to(delete))
        .app_data(web::Data::from(factory))
}

async fn get_all(factory: web::Data<dyn Business>) -> Result<HttpResponse, Error> {
    let result = factory.get_all().await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn get_by_id(
    factory: web::Data<dyn Business>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let result = factory.get_by_id(id).await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn insert(
    factory: web::Data<dyn Business>,
    req: web::Json<StateRequest>,
) -> Result<HttpResponse, Error> {
    let result = factory.insert(req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(InsertResponse { is_success: result }))
}

async fn update(
    factory: web::Data<dyn Business>,
    req: web::Json<StateRequest>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let result = factory.update(path.into_inner(), req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(InsertResponse { is_success: result }))
}

async fn delete(
    factory: web::Data<dyn Business>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let result = factory.delete(path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(InsertResponse { is_success: result }))
}
