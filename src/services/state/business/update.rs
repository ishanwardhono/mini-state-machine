use crate::services::state::{model::StateRequest, repo::DbRepo};
use std::sync::Arc;

pub async fn execute(
    repo: Arc<dyn DbRepo>,
    id: i32,
    state: StateRequest,
) -> Result<bool, sqlx::Error> {
    repo.update(id, state).await
}
