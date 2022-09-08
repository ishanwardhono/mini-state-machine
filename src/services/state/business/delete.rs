use std::sync::Arc;

use crate::{cores::error::Error, services::state::repo::db::DbRepo};

pub async fn execute(repo: Arc<dyn DbRepo>, id: i32) -> Result<bool, Error> {
    validate(id)?;
    repo.delete(id).await
}

fn validate(id: i32) -> Result<(), Error> {
    if id <= 0 {
        return Err(Error::BadRequest("ID is empty".to_string()));
    }
    Ok(())
}
