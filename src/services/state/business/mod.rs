use crate::cores::error::Error;

use super::{
    model::{
        entity::State,
        request::{StateCreateRequest, StateUpdateRequest},
    },
    repo::db::DbRepo,
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
    async fn get_by_id(&self, code: &String) -> Result<State, Error>;
    async fn insert(&self, state: &StateCreateRequest) -> Result<State, Error>;
    async fn update(&self, code: &String, state: StateUpdateRequest) -> Result<String, Error>;
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
        get_all::execute(self.repo.clone()).await
    }
    async fn get_by_id(&self, code: &String) -> Result<State, Error> {
        get_by_id::execute(self.repo.clone(), code).await
    }
    async fn insert(&self, state: &StateCreateRequest) -> Result<State, Error> {
        insert::execute(self.repo.clone(), state).await
    }
    async fn update(&self, code: &String, state: StateUpdateRequest) -> Result<String, Error> {
        update::execute(self.repo.clone(), code, state).await
    }
    async fn delete(&self, code: &String) -> Result<String, Error> {
        delete::execute(self.repo.clone(), code).await
    }
}
