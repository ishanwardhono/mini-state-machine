use super::{delete, get, get_active, insert, valid_creation, valid_transition};
use crate::{
    cores::error::service::Error,
    services::{
        diagram::{model::model::Diagram, repo::db::DbRepo},
        state::StateServiceLogic,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct Factory {
    pub repo: Arc<dyn DbRepo>,
    pub state_logic: Arc<StateServiceLogic>,
}

pub trait Logic: OperationLogic + DiagramLogic {}
impl Logic for Factory {}

#[async_trait]
pub trait OperationLogic {
    async fn insert(&self, req: &Diagram, actor: &Uuid) -> Result<String, Error>;
    async fn get(&self, code: &str) -> Result<Diagram, Error>;
    async fn get_active(&self, code: &str) -> Result<Diagram, Error>;
    async fn delete(&self, code: &str) -> Result<(), Error>;
}

#[async_trait]
pub trait DiagramLogic: Send + Sync {
    async fn valid_transition(&self, code: &str, from: &str, to: &str) -> Result<(), Error>;
    async fn valid_creation(&self, code: &str, state: &str) -> Result<(), Error>;
}

#[async_trait]
impl OperationLogic for Factory {
    async fn insert(&self, req: &Diagram, actor: &Uuid) -> Result<String, Error> {
        tracing::info!("Logic Execute - Insert Diagram");
        insert::execute(self.repo.clone(), self.state_logic.clone(), req, actor).await
    }

    async fn get(&self, code: &str) -> Result<Diagram, Error> {
        tracing::info!("Logic Execute - Get Diagram");
        get::execute(self.repo.clone(), code).await
    }

    async fn get_active(&self, code: &str) -> Result<Diagram, Error> {
        tracing::info!("Logic Execute - Get Active Diagram");
        get_active::execute(self, code).await
    }

    async fn delete(&self, code: &str) -> Result<(), Error> {
        tracing::info!("Logic Execute - Delete Diagram");
        delete::execute(self.repo.clone(), code).await
    }
}

#[async_trait]
impl DiagramLogic for Factory {
    async fn valid_transition(&self, code: &str, from: &str, to: &str) -> Result<(), Error> {
        tracing::info!("Logic Execute - Valid Transition in Diagram");
        valid_transition::execute(self, code, from, to).await
    }

    async fn valid_creation(&self, code: &str, state: &str) -> Result<(), Error> {
        tracing::info!("Logic Execute - Valid Creation in Diagram");
        valid_creation::execute(self, code, state).await
    }
}
