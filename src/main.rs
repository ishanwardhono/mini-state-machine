#[macro_use]

extern crate diesel;
use actix_web::{App, web, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod services;
mod cores;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //environment
    cores::environment::set_env();

    //database
    let manager = ConnectionManager::<PgConnection>::new(
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    );
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool");

    //server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(services::state::get_states)
    })
    .bind(std::env::var("APP_URL").expect("APP_URL must be set"))?
    .run()
    .await
}
