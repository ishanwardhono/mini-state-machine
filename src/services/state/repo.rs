use super::model::State;
use crate::cores::database::DbPool;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Clone)]
pub struct Repo {
    pool: DbPool
}

impl Repo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_states(&self) -> Result<Vec<State>, sqlx::Error> {
        sqlx::query("SELECT * FROM states")
            .map(|row: PgRow| State {
                id: row.get("id"),
                code: row.get("code"),
                description: row.get("description"),
                webhooks: row.get("webhooks"),
                created_at: row.get("created_at"),
            }).fetch_all(&self.pool).await
    }
}
