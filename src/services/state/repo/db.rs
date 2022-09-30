use super::db_query;
use crate::cores::database::pg::{db_time_now, DbPool, DbQueryArguments};
use crate::cores::error::service::Error;
use crate::services::state::model::entity::State;
use crate::services::state::model::request::{StateCreateRequest, StateUpdateRequest};
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::sync::Arc;

#[derive(Clone)]
pub struct DbRepoImpl {
    pool: Arc<DbPool>,
}

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait DbRepo: Sync + Send {
    async fn get_all(&self) -> Result<Vec<State>, Error>;
    async fn get_by_code(&self, code: &String) -> Result<State, Error>;
    async fn insert(&self, state: &StateCreateRequest) -> Result<State, Error>;
    async fn update(&self, code: &String, state: &StateUpdateRequest) -> Result<State, Error>;
    async fn delete(&self, code: &String) -> Result<String, Error>;
}

impl DbRepoImpl {
    pub fn new(pool: Arc<DbPool>) -> Arc<dyn DbRepo> {
        Arc::new(Self { pool })
    }

    fn state_full_map(&self) -> fn(PgRow) -> State {
        |row: PgRow| State {
            id: row.get("id"),
            code: row.get("code"),
            description: row.get("description"),
            webhooks: row.get("webhooks"),
            create_time: row.get("create_time"),
            update_time: row.get("update_time"),
        }
    }

    fn state_default_bind(
        &self,
        query: DbQueryArguments,
        state: &StateCreateRequest,
    ) -> DbQueryArguments {
        query
            //code
            .bind(state.code.clone())
            //description
            .bind(state.description.clone())
            //webhooks
            .bind(state.webhooks.clone())
            //update_time
            .bind(db_time_now())
    }
}

#[async_trait]
impl DbRepo for DbRepoImpl {
    async fn get_all(&self) -> Result<Vec<State>, Error> {
        tracing::info!("Database Execute - Status GetAll Query");

        let result = sqlx::query(db_query::SELECT_ALL)
            .map(self.state_full_map())
            .fetch_all(self.pool.as_ref())
            .await;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::from_db(e)),
        }
    }

    async fn get_by_code(&self, code: &String) -> Result<State, Error> {
        tracing::info!("Database Execute - Status GetByCode Query");

        sqlx::query(db_query::SELECT_BY_CODE)
            .bind(code)
            .map(self.state_full_map())
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }

    async fn insert(&self, state: &StateCreateRequest) -> Result<State, Error> {
        tracing::info!("Database Execute - Status Insert Query");

        let query = sqlx::query(db_query::INSERT);
        self.state_default_bind(query, state)
            //created_time
            .bind(db_time_now())
            .map(self.state_full_map())
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }

    async fn update(&self, code: &String, state: &StateUpdateRequest) -> Result<State, Error> {
        tracing::info!("Database Execute - Status Update Query");

        sqlx::query(db_query::UPDATE)
            .bind(code)
            .bind(state.description.clone())
            .bind(state.webhooks.clone())
            .bind(db_time_now())
            .map(self.state_full_map())
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }

    async fn delete(&self, code: &String) -> Result<String, Error> {
        tracing::info!("Database Execute - Status Delete Query");

        let result = sqlx::query(db_query::DELETE)
            .bind(code)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))?;

        if result.rows_affected() > 0 {
            Ok(code.clone())
        } else {
            Err(Error::NotFound("State not found".to_owned()))
        }
    }
}
