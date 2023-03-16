use uuid::Uuid;

use super::factory::Logic;
use crate::{
    cores::error::service::Error,
    services::{action::model::Action, state::StateServiceLogic},
};
use std::sync::Arc;

pub async fn execute(
    logic: Arc<dyn Logic>,
    state_logic: Arc<StateServiceLogic>,
    action: Action,
    actor: &Uuid,
) -> Result<(), Error> {
    let state = state_logic.get_by_code(&action.to_state).await?;
    if state.actions.is_none() {
        return Ok(());
    }

    for state_client in state.actions.unwrap() {
        let state_client = state_client.clone();
        let action = action.clone();
        let arc_logic = logic.clone();
        tokio::spawn(async move {
            arc_logic.send(state_client, action).await;
        });
    }

    Ok(())
}
