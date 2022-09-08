use std::sync::Arc;

use crate::{cores::error::Error, services::state::repo::db::DbRepo};

pub async fn execute(repo: Arc<dyn DbRepo>, code: &String) -> Result<bool, Error> {
    validate(code)?;
    repo.delete(code).await
}

fn validate(code: &String) -> Result<(), Error> {
    if code.is_empty() {
        return Err(Error::BadRequest("Code is empty".to_string()));
    }
    Ok(())
}
