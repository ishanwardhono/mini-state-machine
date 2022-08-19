use super::model::State;
use crate::cores::database::DbPool;
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::sync::Arc;

#[derive(Clone)]
pub struct DbRepoImpl {
    pool: Arc<DbPool>,
}

#[async_trait]
pub trait DbRepo: Sync + Send {
    async fn get_all(&self) -> Result<Vec<State>, sqlx::Error>;
    async fn get_by_id(&self, id: i32) -> Result<State, sqlx::Error>;
}

impl DbRepoImpl {
    pub fn new(pool: Arc<DbPool>) -> Arc<dyn DbRepo> {
        Arc::new(Self { pool })
    }
}

#[async_trait]
impl DbRepo for DbRepoImpl {
    async fn get_all(&self) -> Result<Vec<State>, sqlx::Error> {
        sqlx::query("SELECT * FROM states")
            .map(|row: PgRow| State {
                id: row.get("id"),
                code: row.get("code"),
                description: row.get("description"),
                webhooks: row.get("webhooks"),
                created_at: row.get("created_at"),
            })
            .fetch_all(self.pool.as_ref())
            .await
    }

    async fn get_by_id(&self, id: i32) -> Result<State, sqlx::Error> {
        sqlx::query("SELECT * FROM states WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| State {
                id: row.get("id"),
                code: row.get("code"),
                description: row.get("description"),
                webhooks: row.get("webhooks"),
                created_at: row.get("created_at"),
            })
            .fetch_one(self.pool.as_ref())
            .await
    }
}
