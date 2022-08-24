use actix_web::{App, HttpServer};
use std::sync::Arc;

mod cores;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //environment
    cores::environment::set_env();

    //development logger
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let pool =
        cores::database::set_db(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await;

    //server
    HttpServer::new(move || App::new().service(services::provider(Arc::new(pool.clone()))))
        .bind(std::env::var("APP_URL").expect("APP_URL must be set"))?
        .run()
        .await
}
