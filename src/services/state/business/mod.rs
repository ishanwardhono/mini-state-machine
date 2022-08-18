use super::{model::State, repo::Repo};

pub mod get_states;

#[derive(Clone)]
pub struct BusinessFactory {
    repo: Repo,
}

impl BusinessFactory {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<State>, sqlx::Error> {
        get_states::execute(&self.repo).await
    }
}
