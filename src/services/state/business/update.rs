use crate::{
    cores::errors::Error,
    services::state::{model::StateRequest, repo::DbRepo},
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, id: i32, state: StateRequest) -> Result<bool, Error> {
    repo.update(id, state).await
}
