use crate::{
    cores::{
        error::{service::Error, types::AuthError},
        http::middleware::auth::Authority,
    },
    services::{
        auth::model::entity::User,
        state::{
            logic::factory::Logic,
            model::{
                request::{StateCreateRequest, StateUpdateRequest},
                response::{CodeResponse, UpsertResponse},
            },
        },
    },
};
use actix_web::{
    web::{self, get, post, put},
    HttpResponse, Scope,
};
use std::sync::Arc;

pub fn register_handler(factory: Arc<dyn Logic>, auth: Authority) -> Scope {
    web::scope("/states")
        .route("", get().to(get_all).wrap(auth.admin()))
        .route("", post().to(insert).wrap(auth.admin()))
        .route("/{code}", get().to(get_by_code).wrap(auth.admin()))
        .route("/{code}", put().to(update).wrap(auth.admin()))
        .route("/{code}", web::delete().to(delete).wrap(auth.admin()))
        .app_data(web::Data::from(factory))
}

async fn get_all(factory: web::Data<dyn Logic>) -> Result<HttpResponse, Error> {
    let result = factory.get_all().await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn get_by_code(
    factory: web::Data<dyn Logic>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let code = path.into_inner();
    let result = factory.get_by_code(&code).await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn insert(
    factory: web::Data<dyn Logic>,
    req: web::Json<StateCreateRequest>,
    user: Option<web::ReqData<User>>,
) -> Result<HttpResponse, Error> {
    if user.is_none() {
        tracing::error!("{}", AuthError::UserNotProvided);
        return Err(Error::unauth_from(AuthError::UserNotProvided));
    }
    let result = factory.insert(&req.into_inner(), &user.unwrap().id).await?;
    Ok(HttpResponse::Ok().json(UpsertResponse { state: result }))
}

async fn update(
    factory: web::Data<dyn Logic>,
    req: web::Json<StateUpdateRequest>,
    path: web::Path<String>,
    user: Option<web::ReqData<User>>,
) -> Result<HttpResponse, Error> {
    if user.is_none() {
        tracing::error!("{}", AuthError::UserNotProvided);
        return Err(Error::unauth_from(AuthError::UserNotProvided));
    }
    let result = factory
        .update(&path.into_inner(), &req.into_inner(), &user.unwrap().id)
        .await?;
    Ok(HttpResponse::Ok().json(UpsertResponse { state: result }))
}

async fn delete(
    factory: web::Data<dyn Logic>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let result = factory.delete(&path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(CodeResponse { code: result }))
}
