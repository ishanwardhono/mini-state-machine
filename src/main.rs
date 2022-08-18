extern crate sqlx;
use actix_web::{web, App, HttpServer};

mod cores;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //environment
    cores::environment::set_env();

    let pool =
        cores::database::set_db(std::env::var("DATABASE_URL").expect("DATAB&ASE_URL must be set"))
            .await;

    //server
    HttpServer::new(move || App::new().service(services::http_register(pool.clone())))
        .bind(std::env::var("APP_URL").expect("APP_URL must be set"))?
        .run()
        .await
}
