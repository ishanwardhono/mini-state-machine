use crate::{
    cores::{auth::role::Role, env::Config, error::service::Error},
    services::auth::{
        logic::{authorize, get_by_username, insert, is_permitted, login, token_validation},
        model::{entity::User, request::UserCreateRequest},
        repo::db::DbRepo,
    },
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct Factory {
    pub cfg: Arc<Config>,
    pub repo: Arc<dyn DbRepo>,
}

#[async_trait]
#[cfg_attr(test, mockall::automock, allow(dead_code))]
pub trait Logic {
    async fn get_by_username(&self, username: &str) -> Result<User, Error>;
    async fn insert(&self, req: &UserCreateRequest) -> Result<User, Error>;
    async fn login(&self, username: &str) -> Result<String, Error>;
    async fn authorize(&self, token: Option<String>, valid_permission: Role)
        -> Result<User, Error>;
    async fn token_validation(&self, token: &str) -> Result<User, Error>;
    fn is_permitted(&self, valid_permission: Role, user_permission: Role) -> bool;
}

#[async_trait]
impl Logic for Factory {
    async fn get_by_username(&self, username: &str) -> Result<User, Error> {
        tracing::info!("Auth - Get by Username");
        get_by_username::execute(self.repo.clone(), username).await
    }

    async fn insert(&self, req: &UserCreateRequest) -> Result<User, Error> {
        tracing::info!("Auth - Insert new User");
        insert::execute(self.repo.clone(), req).await
    }

    async fn login(&self, username: &str) -> Result<String, Error> {
        tracing::info!("Auth - Login");
        login::execute(self.cfg.clone(), self.repo.clone(), username).await
    }

    async fn authorize(
        &self,
        token: Option<String>,
        valid_permission: Role,
    ) -> Result<User, Error> {
        tracing::info!("Auth - Authorization");
        authorize::execute(self, token, valid_permission).await
    }

    async fn token_validation(&self, token: &str) -> Result<User, Error> {
        tracing::debug!("Auth - Token Validation");
        token_validation::execute(self.cfg.jwt.clone(), self.repo.clone(), token).await
    }

    fn is_permitted(&self, valid_permission: Role, user_permission: Role) -> bool {
        tracing::debug!("Auth - Checking Permission");
        is_permitted::execute(valid_permission, user_permission)
    }
}
