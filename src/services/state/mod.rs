mod model;
mod repo;

use actix_web::{web::{self, ServiceConfig}, get, Responder};
use crate::cores::database::DbPool;

pub fn handler(config: &mut ServiceConfig) {
    config.service(get_states);
}

#[get("/state/register")]
pub async fn get_states(data: web::Data<DbPool>) -> impl Responder  {
    let response_data = repo::get_all_states(data.as_ref());
    format!("{:?}", response_data.await)
}