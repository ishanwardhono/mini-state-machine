use crate::{
    cores::error::service::Error,
    services::diagram::{model::model::Diagram, repo::db::DbRepo},
    utils::validation,
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, code: &String) -> Result<Diagram, Error> {
    tracing::debug!("executing ...");
    validate(code)?;
    repo.get(code).await
}

fn validate(code: &String) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if code.is_empty() {
        validation.add_str("Code is empty");
    }
    validation.check()
}
