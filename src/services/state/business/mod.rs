use super::{
    model::{State, StateRequest},
    repo::DbRepo,
};
use async_trait::async_trait;
use std::sync::Arc;

pub mod get_all;
pub mod get_by_id;
pub mod insert;

pub struct BusinessFactory {
    repo: Arc<dyn DbRepo>,
}

#[async_trait]
pub trait Business {
    async fn get_all(&self) -> Result<Vec<State>, sqlx::Error>;
    async fn get_by_id(&self, id: i32) -> Result<State, sqlx::Error>;
    async fn insert(&self, state: StateRequest) -> Result<bool, sqlx::Error>;
}

impl BusinessFactory {
    pub fn new(repo: Arc<dyn DbRepo>) -> Arc<dyn Business> {
        Arc::new(Self { repo })
    }
}

#[async_trait]
impl Business for BusinessFactory {
    async fn get_all(&self) -> Result<Vec<State>, sqlx::Error> {
        get_all::execute(self.repo.clone()).await
    }
    async fn get_by_id(&self, id: i32) -> Result<State, sqlx::Error> {
        get_by_id::execute(self.repo.clone(), id).await
    }
    async fn insert(&self, state: StateRequest) -> Result<bool, sqlx::Error> {
        insert::execute(self.repo.clone(), state).await
    }
}
