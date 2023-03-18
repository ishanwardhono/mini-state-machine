use crate::{
    cores::error::service::Error,
    services::{
        action::{
            logic::{run, send},
            model::Action,
            repo::db::DbRepo,
        },
        client::ClientServiceLogic,
        state::StateServiceLogic,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct Factory {
    pub repo: Arc<dyn DbRepo>,
    pub state_logic: Arc<StateServiceLogic>,
    pub client_logic: Arc<ClientServiceLogic>,
}

#[async_trait]
pub trait Logic: Send + Sync {
    async fn run(&self, action: Action, actor: &Uuid) -> Result<(), Error>;
    async fn send(&self, client_code: String, action: Action, actor: &Uuid) -> Result<(), Error>;
}

#[async_trait]
impl Logic for Factory {
    async fn run(&self, action: Action, actor: &Uuid) -> Result<(), Error> {
        tracing::info!("Logic Execute - Action Run");
        let logic = Arc::new(self.clone());
        run::execute(logic, self.state_logic.clone(), action, actor).await
    }
    async fn send(&self, client_code: String, action: Action, actor: &Uuid) -> Result<(), Error> {
        tracing::info!("Logic Execute - Action Send");
        send::execute(
            self.repo.clone(),
            self.client_logic.clone(),
            client_code,
            action,
            actor,
        )
        .await
    }
}
