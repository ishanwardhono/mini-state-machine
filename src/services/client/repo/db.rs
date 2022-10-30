use super::db_query;
use crate::cores::database::pg::{db_time_now, DbPool};
use crate::cores::error::service::Error;
use crate::services::client::model::model::ClientModel;
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
struct DbRepository {
    pool: Arc<DbPool>,
}

pub fn new(pool: Arc<DbPool>) -> Arc<dyn DbRepo> {
    Arc::new(DbRepository { pool })
}

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait DbRepo: Send + Sync {
    async fn get_by_code(&self, code: &str) -> Result<ClientModel, Error>;
    async fn get_codes(&self, codes: &Vec<String>) -> Result<Vec<String>, Error>;
    async fn insert(&self, client: &ClientModel, actor: &Uuid) -> Result<String, Error>;
    async fn update(&self, client: &ClientModel, actor: &Uuid) -> Result<String, Error>;
    async fn delete(&self, code: &str) -> Result<String, Error>;
}

#[async_trait]
impl DbRepo for DbRepository {
    async fn get_by_code(&self, code: &str) -> Result<ClientModel, Error> {
        tracing::info!("Database Execute - Client GetByCode Query");

        sqlx::query(db_query::SELECT_BY_CODE)
            .bind(code)
            .map(|row: PgRow| ClientModel {
                code: row.get("code"),
                url: row.get("url"),
            })
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }

    async fn get_codes(&self, codes: &Vec<String>) -> Result<Vec<String>, Error> {
        tracing::info!("Database Execute - Clients GetByCodes Query");

        sqlx::query(db_query::SELECT_BY_CODES)
            .bind(codes.as_slice())
            .map(|row: PgRow| row.get("code"))
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }

    async fn insert(&self, client: &ClientModel, actor: &Uuid) -> Result<String, Error> {
        tracing::info!("Database Execute - Client Insert Query");

        let time_now = db_time_now();

        sqlx::query(db_query::INSERT)
            .bind(uuid::Uuid::new_v4())
            .bind(&client.code) //code
            .bind(&client.url) //url
            .bind(time_now) //create_time
            .bind(actor) //create_by
            .bind(time_now) //create_time
            .bind(actor)
            .map(|row: PgRow| row.get("code"))
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }

    async fn update(&self, client: &ClientModel, actor: &Uuid) -> Result<String, Error> {
        tracing::info!("Database Execute - Client Update Query");

        sqlx::query(db_query::UPDATE)
            .bind(&client.code)
            .bind(&client.url)
            .bind(db_time_now())
            .bind(actor)
            .map(|row: PgRow| row.get("code"))
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }

    async fn delete(&self, code: &str) -> Result<String, Error> {
        tracing::info!("Database Execute - Client Delete Query");

        let result = sqlx::query(db_query::DELETE)
            .bind(code)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))?;

        if result.rows_affected() > 0 {
            tracing::info!("clients hard delete ({})", code);
            Ok(code.to_owned())
        } else {
            Err(Error::NotFound("Client not found".to_owned()))
        }
    }
}
