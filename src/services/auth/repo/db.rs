use super::db_query;
use crate::{
    cores::{
        database::{db_time_now, DbPool},
        error::service::Error,
    },
    services::auth::model::{entity::User, request::UserCreateRequest},
};
use async_trait::async_trait;
use sqlx::{postgres::PgRow, Row};
use std::sync::Arc;
use uuid::Uuid;

pub fn new(pool: Arc<DbPool>) -> Arc<dyn DbRepo> {
    Arc::new(DbRepository { pool })
}

#[derive(Clone)]
struct DbRepository {
    pool: Arc<DbPool>,
}

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait DbRepo: Send + Sync {
    async fn get_by_username(&self, username: &str) -> Result<User, Error>;
    async fn insert(&self, user: &UserCreateRequest, actor: &Uuid) -> Result<User, Error>;
}

impl DbRepository {
    fn user_full_map(&self) -> fn(PgRow) -> User {
        |row: PgRow| User {
            id: row.get("id"),
            username: row.get("username"),
            role: row.get("role"),
            business: row.get("business"),
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

    async fn insert(&self, user: &UserCreateRequest, actor: &Uuid) -> Result<User, Error> {
        tracing::info!("Database Execute - User Insert Query");
        sqlx::query(db_query::INSERT)
            .bind(&user.username)
            .bind(&user.role)
            .bind(&user.business)
            .bind(db_time_now())
            .bind(actor)
            .bind(db_time_now())
            .bind(actor)
            .map(self.user_full_map())
            .fetch_one(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))
    }
}
