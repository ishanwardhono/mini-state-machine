use super::send;
use crate::{
    cores::error::service::Error,
    services::{action::model::Action, client::ClientServiceLogic, state::StateServiceLogic},
};
use std::sync::Arc;

pub async fn execute(
    state_logic: Arc<StateServiceLogic>,
    client_logic: Arc<ClientServiceLogic>,
    action: Action,
) -> Result<(), Error> {
    let state = state_logic.get_by_code(&action.to_state).await?;
    if state.actions.is_none() {
        return Ok(());
    }
    for state_client in state.actions.unwrap() {
        let client_logic = client_logic.clone();
        let state_client = state_client.clone();
        let action = action.clone();
        tokio::spawn(async move {
            send::execute(client_logic, state_client, action).await;
        });
    }

    Ok(())
}
