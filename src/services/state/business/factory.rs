use crate::{
    cores::error::service::Error,
    services::state::{
        business::{delete, get_all, get_by_code, insert, update},
        model::{entity::State, request::StateCreateRequest, request::StateUpdateRequest},
        repo::db::DbRepo,
    },
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct BusinessFactory {
    repo: Arc<dyn DbRepo>,
}

#[async_trait]
pub trait Business {
    async fn get_all(&self) -> Result<Vec<State>, Error>;
    async fn get_by_code(&self, code: &String) -> Result<State, Error>;
    async fn insert(&self, state: &StateCreateRequest) -> Result<State, Error>;
    async fn update(&self, code: &String, state: &StateUpdateRequest) -> Result<State, Error>;
    async fn delete(&self, code: &String) -> Result<String, Error>;
}

impl BusinessFactory {
    pub fn new(repo: Arc<dyn DbRepo>) -> Arc<dyn Business> {
        Arc::new(Self { repo })
    }
}

#[async_trait]
impl Business for BusinessFactory {
    async fn get_all(&self) -> Result<Vec<State>, Error> {
        tracing::info!("Business Execute - Status GetAll");
        get_all::execute(self.repo.clone()).await
    }
    async fn get_by_code(&self, code: &String) -> Result<State, Error> {
        tracing::info!("Business Execute - Status GetById");
        get_by_code::execute(self.repo.clone(), code).await
    }
    async fn insert(&self, state: &StateCreateRequest) -> Result<State, Error> {
        tracing::info!("Business Execute - Status Insert");
        insert::execute(self.repo.clone(), state).await
    }
    async fn update(&self, code: &String, state: &StateUpdateRequest) -> Result<State, Error> {
        tracing::info!("Business Execute - Status Update");
        update::execute(self.repo.clone(), code, state).await
    }
    async fn delete(&self, code: &String) -> Result<String, Error> {
        tracing::info!("Business Execute - Status Delete");
        delete::execute(self.repo.clone(), code).await
    }
}
