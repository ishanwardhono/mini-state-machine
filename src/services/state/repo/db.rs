use crate::cores::database::{db_time_now, DbPool, DbQueryArguments};
use crate::cores::error::Error;
use crate::services::state::model::entity::State;
use crate::services::state::model::request::StateRequest;
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
    async fn get_by_id(&self, id: i32) -> Result<State, Error>;
    async fn insert(&self, state: StateRequest) -> Result<bool, Error>;
    async fn update(&self, id: i32, state: StateRequest) -> Result<bool, Error>;
    async fn delete(&self, id: i32) -> Result<bool, Error>;
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

    fn state_default_bind(&self, query: DbQueryArguments, state: StateRequest) -> DbQueryArguments {
        query
            //code
            .bind(state.code)
            //description
            .bind(state.description)
            //webhooks
            .bind(state.webhooks)
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

    async fn get_by_id(&self, id: i32) -> Result<State, Error> {
        let result = sqlx::query(db_query::SELECT_BY_ID)
            .bind(id)
            .map(self.state_full_map())
            .fetch_one(self.pool.as_ref())
            .await;

        match result {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::from_db(e)),
        }
    }

    async fn insert(&self, state: StateRequest) -> Result<bool, Error> {
        let query = sqlx::query(db_query::INSERT);

        let result = self
            .state_default_bind(query, state)
            //created_time
            .bind(db_time_now())
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
        let query = sqlx::query(db_query::UPDATE).bind(id);

        let result = self
            .state_default_bind(query, state)
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
        let result = sqlx::query(db_query::DELETE)
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
