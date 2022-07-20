use actix_web::{web, get, Responder};

use crate::Pool;

mod model;
mod repo;
mod schema;

#[get("/state/register")]
pub async fn get_states(data: web::Data<Pool>) -> impl Responder  {
    let response_data = repo::get_all_states(data);
    format!("{:?}", response_data.unwrap())
}