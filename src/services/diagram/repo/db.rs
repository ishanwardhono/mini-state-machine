use super::db_query;
use crate::{
    cores::{
        database::pg::{db_time_now, DbPool},
        error::service::Error,
    },
    services::diagram::model::model::Diagram,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct DbRepoImpl {
    pool: Arc<DbPool>,
}

#[async_trait]
pub trait DbRepo: Sync + Send {
    async fn insert(&self, diagram: &Diagram, actor: &Uuid) -> Result<(), Error>;
}

impl DbRepoImpl {
    pub fn new(pool: Arc<DbPool>) -> Arc<dyn DbRepo> {
        Arc::new(Self { pool })
    }
}

#[async_trait]
impl DbRepo for DbRepoImpl {
    async fn insert(&self, diagram: &Diagram, actor: &Uuid) -> Result<(), Error> {
        tracing::info!("Database Execute - Diagram Insert Query");

        let time_now = db_time_now();

        let result = sqlx::query(db_query::BUSINESS_INSERT)
            .bind(Uuid::new_v4())
            .bind(diagram.business.code.clone())
            .bind(diagram.business.description.clone())
            .bind(diagram.business.is_active.clone())
            .bind(time_now)
            .bind(actor)
            .bind(time_now)
            .bind(actor)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| Error::from_db(e));
        result.map(|_| {})?;

        for flow in diagram.flows.iter() {
            let result = sqlx::query(db_query::FLOW_INSERT_BULK)
                .bind(Uuid::new_v4())
                .bind(flow.business.clone())
                .bind(flow.state.clone())
                .bind(flow.is_initial_state.clone())
                .bind(flow.next_states.clone())
                .bind(time_now)
                .bind(actor)
                .bind(time_now)
                .bind(actor)
                .execute(self.pool.as_ref())
                .await
                .map_err(|e| Error::from_db(e));
            result.map(|_| {})?
        }
        Ok(())
    }
}
