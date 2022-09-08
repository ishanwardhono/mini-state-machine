use crate::cores::database::{db_time_now, DbPool, DbQueryArguments};
use crate::cores::error::Error;
use crate::services::state::model::entity::State;
use crate::services::state::model::request::{StateCreateRequest, StateUpdateRequest};
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::sync::Arc;

use super::db_query;

#[derive(Clone)]
pub struct DbRepoImpl {
    pool: Arc<DbPool>,
}

#[async_trait]
pub trait DbRepo: Sync + Send {
    async fn get_all(&self) -> Result<Vec<State>, Error>;
    async fn get_by_code(&self, code: &String) -> Result<State, Error>;
    async fn insert(&self, state: &StateCreateRequest) -> Result<State, Error>;
    async fn update(&self, code: &String, state: StateUpdateRequest) -> Result<String, Error>;
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
        let result = sqlx::query(db_query::SELECT_BY_CODE)
            .bind(code)
            .map(self.state_full_map())
            .fetch_one(self.pool.as_ref())
            .await;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::from_db(e)),
        }
    }

    async fn insert(&self, state: &StateCreateRequest) -> Result<State, Error> {
        let query = sqlx::query(db_query::INSERT);

        let result = self
            .state_default_bind(query, state)
            //created_time
            .bind(db_time_now())
            .map(self.state_full_map())
            .fetch_one(self.pool.as_ref())
            .await;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::from_db(e)),
        }
    }

    async fn update(&self, code: &String, state: StateUpdateRequest) -> Result<String, Error> {
        let result = sqlx::query(db_query::UPDATE)
            .bind(code)
            .bind(state.description)
            .bind(state.webhooks)
            .bind(db_time_now())
            .execute(self.pool.as_ref())
            .await;

        match result {
            Ok(res) => {
                if res.rows_affected() > 0 {
                    Ok(code.clone())
                } else {
                    Err(Error::NotFound("State not found".to_string()))
                }
            }
            Err(e) => Err(Error::from_db(e)),
        }
    }

    async fn delete(&self, code: &String) -> Result<String, Error> {
        let result = sqlx::query(db_query::DELETE)
            .bind(code)
            .execute(self.pool.as_ref())
            .await;

        match result {
            Ok(res) => {
                if res.rows_affected() > 0 {
                    Ok(code.clone())
                } else {
                    Err(Error::NotFound("State not found".to_string()))
                }
            }
            Err(e) => Err(Error::from_db(e)),
        }
    }
}
