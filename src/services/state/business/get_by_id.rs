use crate::services::state::{model::State, repo::DbRepo};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, id: i32) -> Result<State, sqlx::Error> {
    repo.get_by_id(id).await
}
