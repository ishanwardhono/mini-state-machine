use crate::services::state::{
    model::{State, StateRequest},
    repo::DbRepo,
};
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>, state: StateRequest) -> Result<bool, sqlx::Error> {
    repo.insert(state).await
}
