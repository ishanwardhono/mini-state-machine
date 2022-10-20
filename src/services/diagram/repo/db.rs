use super::db_query;
use crate::{
    cores::{
        database::pg::{db_time_now, DbPool},
        error::service::Error,
    },
    services::diagram::model::model::{Diagram, FlowModel},
};
use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

struct DbRepository {
    pool: Arc<DbPool>,
}

pub fn new(pool: Arc<DbPool>) -> Arc<dyn DbRepo> {
    Arc::new(DbRepository { pool })
}

#[async_trait]
pub trait DbRepo: Sync + Send {
    async fn insert(&self, diagram: &Diagram, actor: &Uuid) -> Result<(), Error>;
    async fn get(&self, code: &String) -> Result<Diagram, Error>;
    async fn delete(&self, code: &String) -> Result<(), Error>;
}

#[async_trait]
impl DbRepo for DbRepository {
    async fn insert(&self, diagram: &Diagram, actor: &Uuid) -> Result<(), Error> {
        tracing::info!("Database Execute - Diagram Insert Query");

        let mut tx = self.pool.begin().await?;
        let time_now = db_time_now();

        sqlx::query(db_query::BUSINESS_INSERT)
            .bind(Uuid::new_v4())
            .bind(&diagram.code)
            .bind(&diagram.description)
            .bind(&diagram.is_active)
            .bind(time_now)
            .bind(actor)
            .bind(time_now)
            .bind(actor)
            .execute(&mut tx)
            .await?;

        for flow in diagram.flows.iter() {
            sqlx::query(db_query::FLOW_INSERT)
                .bind(Uuid::new_v4())
                .bind(&diagram.code)
                .bind(&flow.state)
                .bind(&flow.is_initial_state)
                .bind(&flow.transitions)
                .bind(time_now)
                .bind(actor)
                .bind(time_now)
                .bind(actor)
                .execute(&mut tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn get(&self, code: &String) -> Result<Diagram, Error> {
        tracing::info!("Database Execute - Diagram Get Query");

        let mut business = sqlx::query(db_query::BUSINESS_SELECT)
            .bind(&code)
            .map(|row: PgRow| Diagram {
                code: row.get("code"),
                description: row.get("description"),
                is_active: row.get("is_active"),
                flows: vec![],
            })
            .fetch_one(self.pool.as_ref())
            .await?;

        let flows = sqlx::query(db_query::FLOW_SELECT)
            .bind(&code)
            .map(|row: PgRow| FlowModel {
                state: row.get("state"),
                is_initial_state: row.get("is_initial_state"),
                transitions: row.get("transitions"),
            })
            .fetch_all(self.pool.as_ref())
            .await?;

        business.flows = flows;
        Ok(business)
    }

    async fn delete(&self, code: &String) -> Result<(), Error> {
        tracing::info!("Database Execute - Diagram Delete Query");

        let mut tx = self.pool.begin().await?;

        sqlx::query(db_query::BUSINESS_DELETE)
            .bind(&code)
            .execute(&mut tx)
            .await?;
        sqlx::query(db_query::FLOW_DELETE)
            .bind(&code)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}
