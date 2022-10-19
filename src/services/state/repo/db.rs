use super::db_query;
use crate::cores::database::pg::{db_time_now, DbPool};
use crate::cores::error::service::Error;
use crate::services::state::model::entity::State;
use crate::services::state::model::request::{StateCreateRequest, StateUpdateRequest};
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct DbRepoImpl {
    pool: Arc<DbPool>,
}

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait DbRepo: Sync + Send {
    async fn get_all(&self) -> Result<Vec<State>, Error>;
    async fn get_by_code(&self, code: &String) -> Result<State, Error>;
    async fn get_by_codes(&self, codes: &Vec<String>) -> Result<Vec<String>, Error>;
    async fn insert(&self, state: &StateCreateRequest, actor: &Uuid) -> Result<State, Error>;
    async fn update(
        &self,
        code: &String,
        state: &StateUpdateRequest,
        actor: &Uuid,
    ) -> Result<State, Error>;
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
            create_by: row.get("create_by"),
            update_time: row.get("update_time"),
            update_by: row.get("update_by"),
        }
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

    async fn get_by_codes(&self, codes: &Vec<String>) -> Result<Vec<String>, Error> {
        tracing::info!("Database Execute - Status GetByCodes Query");

        sqlx::query(db_query::SELECT_BY_CODES)
            .bind(codes.as_slice())
            .map(|row: PgRow| row.get("code"))
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }

    async fn insert(&self, state: &StateCreateRequest, actor: &Uuid) -> Result<State, Error> {
        tracing::info!("Database Execute - Status Insert Query");

        let time_now = db_time_now();

        sqlx::query(db_query::INSERT)
            .bind(uuid::Uuid::new_v4())
            .bind(state.code.clone()) //code
            .bind(state.description.clone()) //description
            .bind(state.webhooks.clone()) //webhooks
            .bind(time_now) //create_time
            .bind(actor) //create_by
            .bind(time_now) //create_time
            .bind(actor) //update_by
            .map(self.state_full_map())
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }

    async fn update(
        &self,
        code: &String,
        state: &StateUpdateRequest,
        actor: &Uuid,
    ) -> Result<State, Error> {
        tracing::info!("Database Execute - Status Update Query");

        sqlx::query(db_query::UPDATE)
            .bind(code)
            .bind(state.description.clone())
            .bind(state.webhooks.clone())
            .bind(db_time_now())
            .bind(actor)
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
            tracing::info!("states hard delete ({})", code);
            Ok(code.clone())
        } else {
            Err(Error::NotFound("State not found".to_owned()))
        }
    }
}
