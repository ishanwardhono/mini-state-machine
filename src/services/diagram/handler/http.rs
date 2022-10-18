use crate::{
    cores::{error::service::Error, http::middleware::auth::Authority},
    services::diagram::logic::factory::Logic,
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

async fn insert(factory: web::Data<dyn Logic>) -> Result<HttpResponse, Error> {
    let _ = factory.insert().await?;
    Ok(HttpResponse::Ok().json(true))
}
