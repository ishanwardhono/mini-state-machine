use actix_web::web::ServiceConfig;

mod state;

//Http Handler Registration
pub fn http_register(config: &mut ServiceConfig) {
    config.service(state::handler::http::register_handler());
}