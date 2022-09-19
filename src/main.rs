use actix_web::{self, web, App, HttpResponse, HttpServer};
use std::sync::Arc;

mod cores;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //environment
    cores::environment::set_env();

    //log
    let _log_guard = cores::log::init();

    let pool = cores::database::pg::init().await;

    let app_url = std::env::var("APP_URL").expect("APP_URL must be set");

    //server
    tracing::info!("Server Started on {}", app_url);
    HttpServer::new(move || {
        App::new()
            .wrap(cores::http::middleware::HttpMiddleware {})
            .service(services::provider::register(Arc::new(pool.clone())))
            .route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .bind(app_url)?
    .run()
    .await
}
