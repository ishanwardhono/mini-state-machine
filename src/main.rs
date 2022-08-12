extern crate sqlx;
use actix_web::{App, HttpServer, web};

mod services;
mod cores;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //environment
    cores::environment::set_env();

    let pool = cores::database::set_db(std::env::var("DATABASE_URL").expect("DATAB&ASE_URL must be set")).await;

    //server
    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(pool.clone())
            )
            .service(services::state::get_states)
    })
    .bind(std::env::var("APP_URL").expect("APP_URL must be set"))?
    .run()
    .await
}
