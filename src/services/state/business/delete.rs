use std::sync::Arc;

use crate::{cores::errors::Error, services::state::repo::DbRepo};

pub async fn execute(repo: Arc<dyn DbRepo>, id: i32) -> Result<bool, Error> {
    repo.delete(id).await
}
