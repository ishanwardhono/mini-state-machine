use crate::{
    cores::error::Error,
    services::state::{model::request::StateRequest, repo::db::DbRepo},
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, id: i32, state: StateRequest) -> Result<bool, Error> {
    repo.update(id, state).await
}
