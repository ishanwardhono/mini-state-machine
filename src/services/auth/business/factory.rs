use crate::{
    cores::error::Error,
    services::auth::{
        business::{get_by_username, insert},
        model::{entity::User, request::UserCreateRequest},
        repo::db::DbRepo,
    },
};
use std::sync::Arc;

use async_trait::async_trait;
pub struct BusinessFactory {
    repo: Arc<dyn DbRepo>,
}

#[async_trait]
pub trait Business {
    async fn get_by_username(&self, username: &String) -> Result<User, Error>;
    async fn insert(&self, req: &UserCreateRequest) -> Result<User, Error>;
}

impl BusinessFactory {
    pub fn new(repo: Arc<dyn DbRepo>) -> Arc<dyn Business> {
        Arc::new(Self { repo })
    }
}

#[async_trait]
impl Business for BusinessFactory {
    async fn get_by_username(&self, username: &String) -> Result<User, Error> {
        tracing::info!("Auth - Get by Username");
        get_by_username::execute(self.repo.clone(), username).await
    }

    async fn insert(&self, req: &UserCreateRequest) -> Result<User, Error> {
        tracing::info!("Auth - Insert new User");
        insert::execute(self.repo.clone(), req).await
    }
}
