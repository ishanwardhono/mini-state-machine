use crate::{
    cores::error::Error,
    services::state::{model::entity::State, repo::db::DbRepo},
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, code: &String) -> Result<State, Error> {
    validate(code)?;
    repo.get_by_code(code).await
}

fn validate(code: &String) -> Result<(), Error> {
    if code.is_empty() {
        return Err(Error::BadRequest("ID is empty".to_string()));
    }
    Ok(())
}
