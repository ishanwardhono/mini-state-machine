#[macro_use]

extern crate diesel;
use actix_web::{App, web, HttpServer};

mod services;
mod cores;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //environment
    cores::environment::set_env();

    //server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(cores::database::set_db()))
            .service(services::state::get_states)
    })
    .bind(std::env::var("APP_URL").expect("APP_URL must be set"))?
    .run()
    .await
}
