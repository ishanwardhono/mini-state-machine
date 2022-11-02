use crate::{
    cores::error::service::Error,
    services::{
        action::{logic::run, model::Action},
        client::ClientServiceLogic,
        state::StateServiceLogic,
    },
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct Factory {
    pub state_logic: Arc<StateServiceLogic>,
    pub client_logic: Arc<ClientServiceLogic>,
}

#[async_trait]
pub trait Logic: Send + Sync {
    async fn run(&self, action: Action) -> Result<(), Error>;
}

#[async_trait]
impl Logic for Factory {
    async fn run(&self, action: Action) -> Result<(), Error> {
        tracing::info!("Logic Execute - Action Run");
        run::execute(self.state_logic.clone(), self.client_logic.clone(), action).await
    }
}
