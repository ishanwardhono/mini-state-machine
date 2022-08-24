use super::model::{State, StateRequest};
use crate::cores::database::DbPool;
use crate::cores::errors::Error;
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
    async fn get_all(&self) -> Result<Vec<State>, Error>;
    async fn get_by_id(&self, id: i32) -> Result<State, Error>;
    async fn insert(&self, state: StateRequest) -> Result<bool, Error>;
    async fn update(&self, id: i32, state: StateRequest) -> Result<bool, Error>;
    async fn delete(&self, id: i32) -> Result<bool, Error>;
}

impl DbRepoImpl {
    pub fn new(pool: Arc<DbPool>) -> Arc<dyn DbRepo> {
        Arc::new(Self { pool })
    }
}

#[async_trait]
impl DbRepo for DbRepoImpl {
    async fn get_all(&self) -> Result<Vec<State>, Error> {
        let result = sqlx::query("SELECT * FROM states")
            .map(|row: PgRow| State {
                id: row.get("id"),
                code: row.get("code"),
                description: row.get("description"),
                webhooks: row.get("webhooks"),
                created_at: row.get("created_at"),
            })
            .fetch_all(self.pool.as_ref())
            .await;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::from_db(e)),
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<State, Error> {
        let result = sqlx::query("SELECT * FROM states WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| State {
                id: row.get("id"),
                code: row.get("code"),
                description: row.get("description"),
                webhooks: row.get("webhooks"),
                created_at: row.get("created_at"),
            })
            .fetch_one(self.pool.as_ref())
            .await;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::from_db(e)),
        }
    }

    async fn insert(&self, state: StateRequest) -> Result<bool, Error> {
        let result =
            sqlx::query("INSERT INTO states (code, description, webhooks) VALUES ($1,$2,$3)")
                .bind(state.code)
                .bind(state.description)
                .bind(state.webhooks)
                .execute(self.pool.as_ref())
                .await;

        match result {
            Ok(res) => {
                if res.rows_affected() > 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(Error::from_db(e)),
        }
    }

    async fn update(&self, id: i32, state: StateRequest) -> Result<bool, Error> {
        let result = sqlx::query(
            "UPDATE states SET (code, description, webhooks) = ($2,$3,$4) WHERE id = $1",
        )
        .bind(id)
        .bind(state.code)
        .bind(state.description)
        .bind(state.webhooks)
        .execute(self.pool.as_ref())
        .await;

        match result {
            Ok(res) => {
                if res.rows_affected() > 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(Error::from_db(e)),
        }
    }

    async fn delete(&self, id: i32) -> Result<bool, Error> {
        let result = sqlx::query("DELETE FROM states WHERE id = $1")
            .bind(id)
            .execute(self.pool.as_ref())
            .await;

        match result {
            Ok(res) => {
                if res.rows_affected() > 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(e) => Err(Error::from_db(e)),
        }
    }
}
