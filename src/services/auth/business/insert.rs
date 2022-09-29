use crate::{
    cores::error::service::Error,
    services::auth::{
        model::{entity::User, request::UserCreateRequest},
        repo::db::DbRepo,
    },
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, req: &UserCreateRequest) -> Result<User, Error> {
    tracing::debug!("executing...");
    repo.insert(req).await
}
