use actix_web::web::ServiceConfig;

pub mod state;

pub fn register_handlers(config: &mut ServiceConfig) {
    config.configure(state::handler);
}