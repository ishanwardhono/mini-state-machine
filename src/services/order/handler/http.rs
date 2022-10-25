use crate::{
    cores::{
        error::{service::Error, types::AuthError},
        http::middleware::auth::Authority,
    },
    services::{
        auth::model::entity::User,
        order::{
            logic::factory::Logic,
            model::request::{OrderRequest, OrderStateUpdateRequest},
        },
    },
};
use actix_web::{
    web::{self, get, post, put},
    HttpResponse, Scope,
};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

pub fn register_handler(factory: Arc<dyn Logic>, auth: Authority) -> Scope {
    web::scope("/orders")
        .route("", post().to(insert).wrap(auth.business_client()))
        .route("", put().to(upsert).wrap(auth.business_client()))
        .route(
            "state-update",
            post().to(state_update).wrap(auth.business_client()),
        )
        .route("{id}", get().to(get_order).wrap(auth.business_client()))
        .app_data(web::Data::from(factory))
}

async fn insert(
    factory: web::Data<dyn Logic>,
    req: web::Json<OrderRequest>,
    user: Option<web::ReqData<User>>,
) -> Result<HttpResponse, Error> {
    if user.is_none() {
        tracing::error!("{}", AuthError::UserNotProvided);
        return Err(Error::unauth_from(AuthError::UserNotProvided));
    }
    let result = factory.insert(&req.into_inner(), &user.unwrap().id).await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn upsert(
    factory: web::Data<dyn Logic>,
    req: web::Json<OrderRequest>,
    user: Option<web::ReqData<User>>,
) -> Result<HttpResponse, Error> {
    if user.is_none() {
        tracing::error!("{}", AuthError::UserNotProvided);
        return Err(Error::unauth_from(AuthError::UserNotProvided));
    }
    let result = factory.upsert(&req.into_inner(), &user.unwrap().id).await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn state_update(
    factory: web::Data<dyn Logic>,
    req: web::Json<OrderStateUpdateRequest>,
    user: Option<web::ReqData<User>>,
) -> Result<HttpResponse, Error> {
    if user.is_none() {
        tracing::error!("{}", AuthError::UserNotProvided);
        return Err(Error::unauth_from(AuthError::UserNotProvided));
    }
    let result = factory
        .state_update(&req.into_inner(), &user.unwrap().id)
        .await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn get_order(
    factory: web::Data<dyn Logic>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let id = Uuid::from_str(&path)?;
    let result = factory.get_detail(&id).await?;
    Ok(HttpResponse::Ok().json(result))
}
