use crate::{
    cores::{auth::role::Role, error::service::Error},
    services::auth::{
        business::{authorize, check_permission, get_by_username, insert, login, token_validation},
        model::{entity::User, request::UserCreateRequest},
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
    async fn get_by_username(&self, username: &String) -> Result<User, Error>;
    async fn insert(&self, req: &UserCreateRequest) -> Result<User, Error>;
    async fn login(&self, username: &String) -> Result<String, Error>;
    async fn authorize(&self, token: Option<String>, valid_permission: Role) -> Result<i32, Error>;
    async fn token_validation(&self, token: &String) -> Result<User, Error>;
    fn check_permission(&self, valid_permission: Role, user_permission: Role) -> bool;
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

    async fn login(&self, username: &String) -> Result<String, Error> {
        tracing::info!("Auth - Login");
        login::execute(self.repo.clone(), username).await
    }

    async fn authorize(&self, token: Option<String>, valid_permission: Role) -> Result<i32, Error> {
        tracing::info!("Auth - Authorization");
        authorize::execute(self, token, valid_permission).await
    }

    async fn token_validation(&self, token: &String) -> Result<User, Error> {
        tracing::debug!("Auth - Token Validation");
        token_validation::execute(self.repo.clone(), token).await
    }

    fn check_permission(&self, valid_permission: Role, user_permission: Role) -> bool {
        tracing::debug!("Auth - Checking Permission");
        check_permission::execute(valid_permission, user_permission)
    }
}
