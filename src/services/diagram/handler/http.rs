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
    web::{self, delete, get, post},
    HttpResponse, Scope,
};
use std::sync::Arc;

pub fn register_handler(factory: Arc<dyn Logic>, auth: Authority) -> Scope {
    web::scope("/diagrams")
        .route("", post().to(insert).wrap(auth.admin()))
        .route(
            "/{code}",
            get().to(get_diagram).wrap(auth.business_client()),
        )
        .route("/{code}", delete().to(delete_diagram).wrap(auth.admin()))
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
    factory.insert(&req.into_inner(), &user.unwrap().id).await?;
    Ok(HttpResponse::Ok().finish())
}

async fn get_diagram(
    factory: web::Data<dyn Logic>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let code = path.into_inner();
    let result = factory.get(&code).await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn delete_diagram(
    factory: web::Data<dyn Logic>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let code = path.into_inner();
    factory.delete(&code).await?;
    Ok(HttpResponse::Ok().finish())
}
