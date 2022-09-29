use crate::{
    cores::{error::Error, http::middleware::auth::Authority},
    services::state::{
        business::factory::Business,
        model::{
            request::{StateCreateRequest, StateUpdateRequest},
            response::{CodeResponse, UpsertResponse},
        },
    },
};
use actix_web::{
    web::{self, get, post, put},
    HttpResponse, Scope,
};
use std::sync::Arc;

pub fn register_handler(factory: Arc<dyn Business>, auth: Authority) -> Scope {
    web::scope("/states")
        .route("", get().to(get_all).wrap(auth.admin()))
        .route("", post().to(insert).wrap(auth.admin()))
        .route("/{code}", get().to(get_by_code).wrap(auth.admin()))
        .route("/{code}", put().to(update).wrap(auth.admin()))
        .route("/{code}", web::delete().to(delete).wrap(auth.admin()))
        .app_data(web::Data::from(factory))
}

async fn get_all(factory: web::Data<dyn Business>) -> Result<HttpResponse, Error> {
    let result = factory.get_all().await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn get_by_code(
    factory: web::Data<dyn Business>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let code = path.into_inner();
    let result = factory.get_by_code(&code).await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn insert(
    factory: web::Data<dyn Business>,
    req: web::Json<StateCreateRequest>,
) -> Result<HttpResponse, Error> {
    let result = factory.insert(&req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(UpsertResponse { state: result }))
}

async fn update(
    factory: web::Data<dyn Business>,
    req: web::Json<StateUpdateRequest>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let result = factory
        .update(&path.into_inner(), &req.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(UpsertResponse { state: result }))
}

async fn delete(
    factory: web::Data<dyn Business>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let result = factory.delete(&path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(CodeResponse { code: result }))
}
