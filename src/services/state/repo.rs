use actix_web::web;
use super::model::State;
use crate::Pool;
use crate::cores::database::schema::states::dsl::*;
use crate::diesel::RunQueryDsl;

pub fn get_all_states(pool: web::Data<Pool>) -> Result<Vec<State>, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let items = states.load::<State>(&conn)?;

    Ok(items)
}