use crate::cores::error::Error;
use crate::services::state::model::entity::State;
use crate::services::state::repo::db::DbRepo;
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>) -> Result<Vec<State>, Error> {
    tracing::debug!("executing ...");
    repo.get_all().await
}
