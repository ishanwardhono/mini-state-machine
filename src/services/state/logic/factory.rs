use crate::{
    cores::error::service::Error,
    services::{
        client::ClientServiceLogic,
        state::{
            logic::{delete, get_all, get_by_code, get_codes, insert, update},
            model::{entity::State, request::StateCreateRequest, request::StateUpdateRequest},
            repo::db::DbRepo,
        },
    },
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct Factory {
    pub repo: Arc<dyn DbRepo>,
    pub client_logic: Arc<ClientServiceLogic>,
}

#[async_trait]
#[cfg_attr(test, mockall::automock, allow(dead_code))]
pub trait Logic: Send + Sync {
    async fn get_all(&self) -> Result<Vec<State>, Error>;
    async fn get_by_code(&self, code: &str) -> Result<State, Error>;
    async fn get_codes(&self, code: &Vec<String>) -> Result<Vec<String>, Error>;
    async fn insert(&self, state: &StateCreateRequest, actor: &uuid::Uuid) -> Result<State, Error>;
    async fn update(
        &self,
        code: &str,
        state: &StateUpdateRequest,
        actor: &uuid::Uuid,
    ) -> Result<State, Error>;
    async fn delete(&self, code: &str) -> Result<String, Error>;
}

#[async_trait]
impl Logic for Factory {
    async fn get_all(&self) -> Result<Vec<State>, Error> {
        tracing::info!("Logic Execute - Status GetAll");
        get_all::execute(self.repo.clone()).await
    }
    async fn get_by_code(&self, code: &str) -> Result<State, Error> {
        tracing::info!("Logic Execute - Status GetByCode");
        get_by_code::execute(self.repo.clone(), code).await
    }
    async fn get_codes(&self, code: &Vec<String>) -> Result<Vec<String>, Error> {
        tracing::info!("Logic Execute - Status GetByCodes");
        get_codes::execute(self.repo.clone(), code).await
    }
    async fn insert(&self, state: &StateCreateRequest, actor: &uuid::Uuid) -> Result<State, Error> {
        tracing::info!("Logic Execute - Status Insert");
        insert::execute(self.repo.clone(), self.client_logic.clone(), state, actor).await
    }
    async fn update(
        &self,
        code: &str,
        state: &StateUpdateRequest,
        actor: &uuid::Uuid,
    ) -> Result<State, Error> {
        tracing::info!("Logic Execute - Status Update");
        update::execute(
            self.repo.clone(),
            self.client_logic.clone(),
            code,
            state,
            actor,
        )
        .await
    }
    async fn delete(&self, code: &str) -> Result<String, Error> {
        tracing::info!("Logic Execute - Status Delete");
        delete::execute(self.repo.clone(), code).await
    }
}
