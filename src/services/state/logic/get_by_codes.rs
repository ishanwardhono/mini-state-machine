use std::sync::Arc;

use crate::{cores::error::service::Error, services::state::repo::db::DbRepo, utils::validation};

pub async fn execute(repo: Arc<dyn DbRepo>, codes: &Vec<String>) -> Result<Vec<String>, Error> {
    tracing::debug!("executing ...");
    validate(codes)?;
    repo.get_by_codes(codes).await
}

fn validate(codes: &Vec<String>) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if codes.len() <= 0 {
        validation.add_str("Code is empty");
    }
    validation.check()
}
