use crate::{
    cores::error::Error,
    services::state::{model::entity::State, repo::DbRepo},
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, id: i32) -> Result<State, Error> {
    validate(id)?;
    repo.get_by_id(id).await
}

fn validate(id: i32) -> Result<(), Error> {
    if id <= 0 {
        return Err(Error::BadRequest("ID is empty".to_string()));
    }
    Ok(())
}
