use std::sync::Arc;

use super::{model::State, repo::Repo};
use async_trait::async_trait;

pub mod get_states;

#[derive(Clone)]
pub struct BusinessFactory {
    repo: Repo,
}

#[async_trait]
pub trait Business {
    async fn get_all(&self) -> Result<Vec<State>, sqlx::Error>;
}

impl BusinessFactory {
    pub fn new(repo: Repo) -> Arc<dyn Business> {
        Arc::new(Self { repo })
    }
}

#[async_trait]
impl Business for BusinessFactory {
    async fn get_all(&self) -> Result<Vec<State>, sqlx::Error> {
        get_states::execute(&self.repo).await
    }
}
