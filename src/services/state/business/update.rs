use crate::{
    cores::error::Error,
    services::state::{model::request::StateUpdateRequest, repo::db::DbRepo},
};
use std::sync::Arc;

pub async fn execute(
    repo: Arc<dyn DbRepo>,
    code: &String,
    state: StateUpdateRequest,
) -> Result<String, Error> {
    repo.update(code, state).await
}
