use crate::cores::errors::Error;

use super::{
    model::{State, StateRequest},
    repo::DbRepo,
};
use async_trait::async_trait;
use std::sync::Arc;

mod delete;
mod get_all;
mod get_by_id;
mod insert;
mod update;

pub struct BusinessFactory {
    repo: Arc<dyn DbRepo>,
}

#[async_trait]
pub trait Business {
    async fn get_all(&self) -> Result<Vec<State>, Error>;
    async fn get_by_id(&self, id: i32) -> Result<State, Error>;
    async fn insert(&self, state: StateRequest) -> Result<bool, Error>;
    async fn update(&self, id: i32, state: StateRequest) -> Result<bool, Error>;
    async fn delete(&self, id: i32) -> Result<bool, Error>;
}

impl BusinessFactory {
    pub fn new(repo: Arc<dyn DbRepo>) -> Arc<dyn Business> {
        Arc::new(Self { repo })
    }
}

#[async_trait]
impl Business for BusinessFactory {
    async fn get_all(&self) -> Result<Vec<State>, Error> {
        get_all::execute(self.repo.clone()).await
    }
    async fn get_by_id(&self, id: i32) -> Result<State, Error> {
        get_by_id::execute(self.repo.clone(), id).await
    }
    async fn insert(&self, state: StateRequest) -> Result<bool, Error> {
        insert::execute(self.repo.clone(), state).await
    }
    async fn update(&self, id: i32, state: StateRequest) -> Result<bool, Error> {
        update::execute(self.repo.clone(), id, state).await
    }
    async fn delete(&self, id: i32) -> Result<bool, Error> {
        delete::execute(self.repo.clone(), id).await
    }
}
