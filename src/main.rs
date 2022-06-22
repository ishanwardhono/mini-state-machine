#[macro_use]

extern crate diesel;
use actix_web::{App, web, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

//services
use services::state;

mod services {
    pub mod state;
}
mod errors;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //environment
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debud");

    //database
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool");

    //server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(state::get_states)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
