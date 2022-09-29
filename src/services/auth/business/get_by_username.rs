use crate::{
    cores::error::service::Error,
    services::auth::{model::entity::User, repo::db::DbRepo},
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, username: &String) -> Result<User, Error> {
    tracing::debug!("executing...");
    repo.get_by_username(username).await
}
