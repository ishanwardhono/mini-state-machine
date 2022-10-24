use super::db_query;
use crate::{
    cores::{
        database::pg::{db_time_now, DbPool},
        error::service::Error,
    },
    services::auth::model::{entity::User, request::UserCreateRequest},
};
use async_trait::async_trait;
use sqlx::{postgres::PgRow, Row};
use std::sync::Arc;

#[derive(Clone)]
pub struct DbRepository {
    pub pool: Arc<DbPool>,
}

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait DbRepo: Sync + Send {
    async fn get_by_username(&self, username: &str) -> Result<User, Error>;
    async fn insert(&self, user: &UserCreateRequest) -> Result<User, Error>;
}

impl DbRepository {
    fn user_full_map(&self) -> fn(PgRow) -> User {
        |row: PgRow| User {
            id: row.get("id"),
            username: row.get("username"),
            role: row.get("role"),
            create_time: row.get("create_time"),
            create_by: row.get("create_by"),
            update_time: row.get("update_time"),
            update_by: row.get("update_by"),
        }
    }
}

#[async_trait]
impl DbRepo for DbRepository {
    async fn get_by_username(&self, username: &str) -> Result<User, Error> {
        tracing::info!("Database Execute - User GetByUsername Query");
        sqlx::query(db_query::GET_BY_USERNAME)
            .bind(username)
            .map(self.user_full_map())
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }

    async fn insert(&self, user: &UserCreateRequest) -> Result<User, Error> {
        tracing::info!("Database Execute - User Insert Query");
        sqlx::query(db_query::INSERT)
            .bind(&user.username)
            .bind(&user.role)
            .bind(db_time_now())
            .bind(db_time_now())
            .map(self.user_full_map())
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }
}
