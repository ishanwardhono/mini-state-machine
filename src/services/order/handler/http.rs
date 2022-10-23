use std::sync::Arc;

use actix_web::{
    web::{self, post},
    HttpResponse, Scope,
};

use crate::{
    cores::{
        error::{service::Error, types::AuthError},
        http::middleware::auth::Authority,
    },
    services::{
        auth::model::entity::User,
        order::{logic::factory::Logic, model::request::OrderRequest},
    },
};

pub fn register_handler(factory: Arc<dyn Logic>, auth: Authority) -> Scope {
    web::scope("/orders")
        .route("", post().to(insert).wrap(auth.business_client()))
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
    factory.insert(&req.into_inner(), &user.unwrap().id).await?;
    Ok(HttpResponse::Ok().finish())
}
