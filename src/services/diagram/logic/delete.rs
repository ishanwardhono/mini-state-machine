use crate::{cores::error::service::Error, services::diagram::repo::db::DbRepo, utils::validation};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, code: &String) -> Result<(), Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    repo.delete(code).await
}

fn validate(code: &String) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if code.is_empty() {
        validation.add_str("Code is empty");
    }
    validation.check()
}
