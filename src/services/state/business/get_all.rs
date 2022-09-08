use crate::cores::error::Error;
use crate::services::state::model::entity::State;
use crate::services::state::repo::DbRepo;
use std::sync::Arc;

pub async fn execute(repo: Arc<dyn DbRepo>) -> Result<Vec<State>, Error> {
    repo.get_all().await
}
