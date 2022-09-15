use std::sync::Arc;

use crate::{
    cores::error::Error, services::state::repo::db::DbRepo, utils::common::FieldValidation,
};

pub async fn execute(repo: Arc<dyn DbRepo>, code: &String) -> Result<String, Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    repo.delete(code).await
}

fn validate(req: &String) -> Result<(), Error> {
    let mut validation = FieldValidation::new();
    if req == "" {
        validation.add("Code is empty");
    }

    validation.check()
}
