use super::factory::OperationLogic;
use crate::{cores::error::service::Error, utils::validation};

pub async fn execute<'a>(
    factory: &impl OperationLogic,
    code: &'a str,
    state: &'a str,
) -> Result<(), Error> {
    tracing::debug!("executing ...");
    validate(code, state)?;
    let mut diagram = factory.get_active(code).await?;

    //call remove bcs we take ownership
    let from_state = diagram
        .flows
        .remove(state)
        .ok_or(Error::BadRequest(format!(
            "State {} not found in diagram",
            state
        )))?;

    if !from_state.is_initial_state {
        return Err(Error::BadRequest(format!(
            "State {} is not initial state",
            state
        )));
    }

    Ok(())
}

fn validate(code: &str, state: &str) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if code.is_empty() {
        validation.add_str("Code is empty");
    }
    if state.is_empty() {
        validation.add_str("State is empty");
    }
    validation.check()
}
