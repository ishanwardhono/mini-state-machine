use crate::{
    cores::error::service::Error,
    services::client::{
        logic::{delete, get_by_code, insert, update},
        model::model::ClientModel,
        repo::db::DbRepo,
    },
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct Factory {
    pub repo: Arc<dyn DbRepo>,
}

#[async_trait]
#[cfg_attr(test, mockall::automock, allow(dead_code))]
pub trait Logic: Send + Sync {
    async fn get_by_code(&self, code: &str) -> Result<ClientModel, Error>;
    async fn insert(&self, client: &ClientModel, actor: &uuid::Uuid) -> Result<String, Error>;
    async fn update(&self, client: &ClientModel, actor: &uuid::Uuid) -> Result<String, Error>;
    async fn delete(&self, code: &str) -> Result<String, Error>;
}

#[async_trait]
impl Logic for Factory {
    async fn get_by_code(&self, code: &str) -> Result<ClientModel, Error> {
        tracing::info!("Logic Execute - Client GetByCode");
        get_by_code::execute(self.repo.clone(), code).await
    }
    async fn insert(&self, client: &ClientModel, actor: &uuid::Uuid) -> Result<String, Error> {
        tracing::info!("Logic Execute - Client Insert");
        insert::execute(self.repo.clone(), client, actor).await
    }
    async fn update(&self, client: &ClientModel, actor: &uuid::Uuid) -> Result<String, Error> {
        tracing::info!("Logic Execute - Client Update");
        update::execute(self.repo.clone(), client, actor).await
    }
    async fn delete(&self, code: &str) -> Result<String, Error> {
        tracing::info!("Logic Execute - Client Delete");
        delete::execute(self.repo.clone(), code).await
    }
}
