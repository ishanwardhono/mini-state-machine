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
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

pub struct DbRepository {
    pub pool: Arc<DbPool>,
}

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait DbRepo: Sync + Send {
    async fn insert(&self, diagram: &Diagram, actor: &Uuid) -> Result<String, Error>;
    async fn get(&self, code: &str) -> Result<Diagram, Error>;
    async fn delete(&self, code: &str) -> Result<(), Error>;
}

#[async_trait]
impl DbRepo for DbRepository {
    async fn insert(&self, diagram: &Diagram, actor: &Uuid) -> Result<String, Error> {
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

        for (state, flow) in &diagram.flows {
            sqlx::query(db_query::FLOW_INSERT)
                .bind(Uuid::new_v4())
                .bind(&diagram.code)
                .bind(&state)
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
        Ok(diagram.code.clone())
    }

    async fn get(&self, code: &str) -> Result<Diagram, Error> {
        tracing::info!("Database Execute - Diagram Get Query");

        let mut business = sqlx::query(db_query::BUSINESS_SELECT)
            .bind(&code)
            .map(|row: PgRow| Diagram {
                code: row.get("code"),
                description: row.get("description"),
                is_active: row.get("is_active"),
                flows: HashMap::new(),
            })
            .fetch_one(self.pool.as_ref())
            .await?;

        sqlx::query(db_query::FLOW_SELECT)
            .bind(&code)
            .map(|row: PgRow| {
                business.flows.insert(
                    row.get("state"),
                    FlowModel {
                        is_initial_state: row.get("is_initial_state"),
                        transitions: row.get("transitions"),
                    },
                )
            })
            .fetch_all(self.pool.as_ref())
            .await?;

        Ok(business)
    }

    async fn delete(&self, code: &str) -> Result<(), Error> {
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
