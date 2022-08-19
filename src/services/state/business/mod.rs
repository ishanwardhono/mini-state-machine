use super::{model::State, repo::DbRepo};
use async_trait::async_trait;
use std::sync::Arc;

pub mod get_states;

pub struct BusinessFactory {
    repo: Arc<dyn DbRepo>,
}

#[async_trait]
pub trait Business {
    async fn get_all(&self) -> Result<Vec<State>, sqlx::Error>;
}

impl BusinessFactory {
    pub fn new(repo: Arc<dyn DbRepo>) -> Arc<dyn Business> {
        Arc::new(Self { repo })
    }
}

#[async_trait]
impl Business for BusinessFactory {
    async fn get_all(&self) -> Result<Vec<State>, sqlx::Error> {
        get_states::execute(self.repo.clone()).await
    }
}
