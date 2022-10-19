use crate::{
    cores::error::service::Error,
    services::state::{
        logic::{delete, get_all, get_by_code, get_by_codes, insert, update},
        model::{entity::State, request::StateCreateRequest, request::StateUpdateRequest},
        repo::db::DbRepo,
    },
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct LogicFactory {
    repo: Arc<dyn DbRepo>,
}

impl LogicFactory {
    pub fn new(repo: Arc<dyn DbRepo>) -> Arc<dyn Logic> {
        Arc::new(Self { repo })
    }
}

#[async_trait]
pub trait Logic: Sync + Send {
    async fn get_all(&self) -> Result<Vec<State>, Error>;
    async fn get_by_code(&self, code: &String) -> Result<State, Error>;
    async fn get_by_codes(&self, code: &Vec<String>) -> Result<Vec<String>, Error>;
    async fn insert(&self, state: &StateCreateRequest, actor: &uuid::Uuid) -> Result<State, Error>;
    async fn update(
        &self,
        code: &String,
        state: &StateUpdateRequest,
        actor: &uuid::Uuid,
    ) -> Result<State, Error>;
    async fn delete(&self, code: &String) -> Result<String, Error>;
}

#[async_trait]
impl Logic for LogicFactory {
    async fn get_all(&self) -> Result<Vec<State>, Error> {
        tracing::info!("Logic Execute - Status GetAll");
        get_all::execute(self.repo.clone()).await
    }
    async fn get_by_code(&self, code: &String) -> Result<State, Error> {
        tracing::info!("Logic Execute - Status GetByCode");
        get_by_code::execute(self.repo.clone(), code).await
    }
    async fn get_by_codes(&self, code: &Vec<String>) -> Result<Vec<String>, Error> {
        tracing::info!("Logic Execute - Status GetByCodes");
        get_by_codes::execute(self.repo.clone(), code).await
    }
    async fn insert(&self, state: &StateCreateRequest, actor: &uuid::Uuid) -> Result<State, Error> {
        tracing::info!("Logic Execute - Status Insert");
        insert::execute(self.repo.clone(), state, actor).await
    }
    async fn update(
        &self,
        code: &String,
        state: &StateUpdateRequest,
        actor: &uuid::Uuid,
    ) -> Result<State, Error> {
        tracing::info!("Logic Execute - Status Update");
        update::execute(self.repo.clone(), code, state, actor).await
    }
    async fn delete(&self, code: &String) -> Result<String, Error> {
        tracing::info!("Logic Execute - Status Delete");
        delete::execute(self.repo.clone(), code).await
    }
}
