use actix_web::{self, web, App, HttpResponse, HttpServer};
use std::sync::Arc;

mod cores;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //environment
    let config = cores::env::Config::set();

    //log
    let _log_guard = cores::log::init(config.log.clone());

    let pool = cores::database::init(config.db.clone()).await;

    let app_url = config.app.url.clone();

    //server
    tracing::info!("Server Started on {}", app_url);
    HttpServer::new(move || {
        App::new()
            .wrap(cores::http::middleware::span::new())
            .service(services::provider::register(
                config.clone(),
                Arc::new(pool.clone()),
            ))
            .route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .bind(app_url)?
    .run()
    .await
}
