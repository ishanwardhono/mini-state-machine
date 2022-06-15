use actix_web::{Responder, web, get};

#[get("/state/register")]
pub async fn get_states(name: web::Path<String>) -> impl Responder {
    format!("Hello this is the states from {name}")
}