mod model;
mod repo;

use actix_web::{web, get, Responder};
use crate::cores::database::DbPool;

#[get("/state/register")]
pub async fn get_states(data: web::Data<DbPool>) -> impl Responder  {
    let response_data = repo::get_all_states(data.as_ref());
    format!("{:?}", response_data.await)
}