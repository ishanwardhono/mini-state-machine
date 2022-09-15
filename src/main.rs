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
    cores::log::init_log();

    let pool = cores::database::set_db().await;

    let app_url = std::env::var("APP_URL").expect("APP_URL must be set");

    //server
    tracing::info!("Server Started on {}", app_url);
    HttpServer::new(move || {
        App::new()
            .wrap(cores::http::middleware::HttpMiddleware {})
            .service(services::provider(Arc::new(pool.clone())))
            .route("/", web::get().to(|| HttpResponse::Ok()))
    })
    .bind(app_url)?
    .run()
    .await
}
