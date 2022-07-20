use actix_web::{web, get, Responder};

use crate::cores::database;

mod model;
mod repo;

#[get("/state/register")]
pub async fn get_states(data: web::Data<database::Pool>) -> impl Responder  {
    let response_data = repo::get_all_states(data);
    format!("{:?}", response_data.unwrap())
}