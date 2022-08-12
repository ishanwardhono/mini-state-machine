use crate::services::state;

use crate::cores::database::DbPool;
use state::model::State;
use state::repo;

pub async fn execute(db: DbPool) -> Result<Vec<State>, sqlx::Error>  {
    repo::get_all_states(&db).await
}