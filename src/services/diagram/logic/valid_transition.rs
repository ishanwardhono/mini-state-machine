use std::sync::Arc;

use crate::{cores::error::service::Error, services::diagram::repo::db::DbRepo, utils::validation};

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    code: &'a String,
    from: &'a String,
    to: &'a String,
) -> Result<(), Error> {
    tracing::debug!("executing ...");
    validate(code, from, to)?;
    let mut diagram = repo.get(&code).await?;

    //call remove bcs we take ownership
    let from_state = diagram
        .flows
        .remove(from)
        .ok_or(Error::BadRequest("From state not found".to_owned()))?;

    let transition = from_state.transitions.ok_or(Error::BadRequest(
        "Transition not found / State is final".to_owned(),
    ))?;

    if !transition.contains(to) {
        return Err(Error::BadRequest(format!(
            "Transition {} does not contain {}",
            from, to
        )));
    }

    Ok(())
}

fn validate(code: &String, from: &String, to: &String) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if code.is_empty() {
        validation.add_str("Code is empty");
    }
    if from.is_empty() {
        validation.add_str("From State is empty");
    }
    if to.is_empty() {
        validation.add_str("To State is empty");
    }
    validation.check()
}
