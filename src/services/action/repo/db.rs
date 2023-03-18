use crate::{
    cores::{
        database::pg::{db_time_now, DbPool},
        error::service::Error,
    },
    services::action::model::InsertRetryAction,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use super::db_query;

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
    async fn insert(&self, retry_action: InsertRetryAction, actor: &Uuid) -> Result<(), Error>;
}

#[async_trait]
impl DbRepo for DbRepository {
    async fn insert(&self, retry_action: InsertRetryAction, actor: &Uuid) -> Result<(), Error> {
        tracing::info!("Database Execute - Status Insert Query");

        let time_now = db_time_now();

        sqlx::query(db_query::INSERT)
            .bind(Uuid::new_v4())
            .bind(&retry_action.client)
            .bind(&retry_action.business)
            .bind(&retry_action.order_id)
            .bind(&retry_action.from_state)
            .bind(&retry_action.to_state)
            .bind(time_now)
            .bind(actor)
            .bind(time_now)
            .bind(actor)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e))?;

        Ok(())
    }
}
