use crate::{
    cores::{
        error::{service::Error, types::AuthError},
        http::middleware::auth::Authority,
    },
    services::{
        auth::model::entity::User,
        diagram::{logic::factory::Logic, model::model::Diagram},
    },
};
use actix_web::{
    web::{self, post},
    HttpResponse, Scope,
};
use std::sync::Arc;

pub fn register_handler(factory: Arc<dyn Logic>, auth: Authority) -> Scope {
    web::scope("/diagrams")
        .route("", post().to(insert).wrap(auth.admin()))
        .app_data(web::Data::from(factory))
}

async fn insert(
    factory: web::Data<dyn Logic>,
    req: web::Json<Diagram>,
    user: Option<web::ReqData<User>>,
) -> Result<HttpResponse, Error> {
    if user.is_none() {
        tracing::error!("{}", AuthError::UserNotProvided);
        return Err(Error::unauth_from(AuthError::UserNotProvided));
    }
    let _ = factory.insert(&req.into_inner(), &user.unwrap().id).await?;
    Ok(HttpResponse::Ok().json(true))
}
