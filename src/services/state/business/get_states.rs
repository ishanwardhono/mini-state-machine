use crate::services::state;
use crate::services::state::repo::Repo;
use state::model::State;

pub async fn execute(repo: &Repo) -> Result<Vec<State>, sqlx::Error> {
    repo.get_all_states().await
}
