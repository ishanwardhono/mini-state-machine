use crate::{
    cores::error::Error,
    services::state::{model::entity::State, repo::db::DbRepo},
    utils::common::FieldValidation,
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, code: &String) -> Result<State, Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    repo.get_by_code(code).await
}

fn validate(req: &String) -> Result<(), Error> {
    let mut validation = FieldValidation::new();
    if req == "" {
        validation.add("Code is empty");
    }

    validation.check()
}
