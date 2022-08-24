use std::sync::Arc;

use crate::services::state::repo::DbRepo;

pub async fn execute(repo: Arc<dyn DbRepo>, id: i32) -> Result<bool, sqlx::Error> {
    repo.delete(id).await
}
