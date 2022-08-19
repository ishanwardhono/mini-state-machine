use crate::services::state;
use crate::services::state::repo::DbRepo;
use state::model::State;
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>) -> Result<Vec<State>, sqlx::Error> {
    repo.get_all().await
}
