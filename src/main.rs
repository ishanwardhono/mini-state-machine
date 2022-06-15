#[macro_use]

use actix_web::{web, App, HttpServer};
use services::state;

mod services {
    pub mod state;
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debud");

    HttpServer::new(move || {
        App::new()
            .service(state::get_states)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
