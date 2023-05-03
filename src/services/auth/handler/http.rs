use crate::{
    cores::{
        error::{service::Error, types::AuthError},
        http::middleware::auth::Authority,
    },
    services::auth::{
        logic::factory::Logic,
        model::{
            entity::User,
            request::UserCreateRequest,
            response::{UserCreateResponse, UserKeyResponse},
        },
    },
};
use actix_web::{
    web::{self, post},
    HttpResponse, Scope,
};
use std::sync::Arc;

pub fn register_handler(factory: Arc<dyn Logic>, auth: Authority) -> Scope {
    web::scope("/users")
        .route("", post().to(insert).wrap(auth.admin()))
        .route(
            "{user}/generate-key",
            post().to(generate_key).wrap(auth.admin()),
        )
        .app_data(web::Data::from(factory))
}

async fn insert(
    factory: web::Data<dyn Logic>,
    req: web::Json<UserCreateRequest>,
    user: Option<web::ReqData<User>>,
) -> Result<HttpResponse, Error> {
    if user.is_none() {
        tracing::error!("{}", AuthError::UserNotProvided);
        return Err(Error::unauth_from(AuthError::UserNotProvided));
    }
    let result = factory.insert(&req, &user.unwrap().id).await?;
    Ok(HttpResponse::Ok().json(UserCreateResponse {
        username: result.username,
    }))
}

async fn generate_key(
    factory: web::Data<dyn Logic>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let result = factory.generate_key(&path).await?;
    Ok(HttpResponse::Ok().json(UserKeyResponse {
        username: path.to_string(),
        key: result,
    }))
}
