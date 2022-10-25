use super::db_query;
use crate::{
    cores::{
        database::pg::{db_time_now, DbPool},
        error::service::Error,
    },
    services::order::model::{
        entity::Order,
        model::{HistoryModel, OrderModel},
        request::OrderRequest,
        response::OrderResponse,
    },
};
use async_trait::async_trait;
use sqlx::Row;
use sqlx::{postgres::PgRow, Executor, Postgres};
use std::sync::Arc;
use uuid::Uuid;

pub fn new(pool: Arc<DbPool>) -> Arc<dyn DbRepo> {
    Arc::new(DbRepository { pool })
}

struct DbRepository {
    pool: Arc<DbPool>,
}

#[async_trait]
pub trait DbRepo: Send + Sync {
    async fn insert(&self, order: &OrderRequest, actor: &Uuid) -> Result<OrderResponse, Error>;
    async fn state_update(
        &self,
        id: &Uuid,
        from_state: &str,
        to_state: &str,
        actor: &Uuid,
    ) -> Result<(), Error>;
    async fn get(&self, id: &Uuid) -> Result<Order, Error>;
    async fn get_by_client_order_id(
        &self,
        business: &str,
        client_order_id: &str,
    ) -> Result<Order, Error>;
    async fn get_detail(&self, id: &Uuid) -> Result<OrderModel, Error>;
    async fn exists_client_order_id(
        &self,
        business: &str,
        client_order_id: &str,
    ) -> Result<bool, Error>;
}

#[async_trait]
impl DbRepo for DbRepository {
    async fn insert(&self, order: &OrderRequest, actor: &Uuid) -> Result<OrderResponse, Error> {
        tracing::info!("Database Execute - Order Creation Query");
        let mut tx = self.pool.begin().await?;

        let time_now = db_time_now();
        let id = Uuid::new_v4();
        let client_order_id = match &order.client_order_id {
            Some(client_order_id) => client_order_id.clone(),
            None => id.to_string(),
        };

        sqlx::query(db_query::ORDER_INSERT)
            .bind(&id)
            .bind(&client_order_id)
            .bind(&order.business)
            .bind(&order.state)
            .bind(time_now)
            .bind(actor)
            .bind(time_now)
            .bind(actor)
            .execute(&mut tx)
            .await?;

        let history = HistoryModel {
            from_state: "".to_owned(),
            to_state: order.state.clone(),
            create_time: time_now,
            create_by: actor.clone(),
        };
        self.insert_history(&mut tx, &id, &history, actor).await?;

        tx.commit().await?;
        Ok(OrderResponse {
            id,
            client_order_id,
            business: order.business.clone(),
            state: order.state.clone(),
        })
    }

    async fn state_update(
        &self,
        id: &Uuid,
        from_state: &str,
        to_state: &str,
        actor: &Uuid,
    ) -> Result<(), Error> {
        tracing::info!("Database Execute - Order Status Update Query");
        let mut tx = self.pool.begin().await?;

        let time_now = db_time_now();
        sqlx::query(db_query::ORDER_STATE_UPDATE)
            .bind(&id)
            .bind(&to_state)
            .bind(time_now)
            .bind(actor)
            .execute(&mut tx)
            .await?;

        let history = HistoryModel {
            from_state: from_state.to_string(),
            to_state: to_state.to_string(),
            create_time: time_now,
            create_by: actor.clone(),
        };
        self.insert_history(&mut tx, &id, &history, actor).await?;

        tx.commit().await?;
        Ok(())
    }

    async fn get(&self, id: &Uuid) -> Result<Order, Error> {
        tracing::info!("Database Execute - Order Get Query");

        let res = sqlx::query(db_query::ORDER_GET)
            .bind(id)
            .map(|row: PgRow| Order {
                id: row.get("id"),
                client_order_id: row.get("client_order_id"),
                business: row.get("business"),
                state: row.get("state"),
                create_time: row.get("create_time"),
                create_by: row.get("create_by"),
                update_time: row.get("update_time"),
                update_by: row.get("update_by"),
            })
            .fetch_one(self.pool.as_ref())
            .await?;
        Ok(res)
    }

    async fn get_by_client_order_id(
        &self,
        business: &str,
        client_order_id: &str,
    ) -> Result<Order, Error> {
        tracing::info!("Database Execute - Order Get By Order Id Query");

        let res = sqlx::query(db_query::ORDER_GET_BY_CLIENT_ORDER_ID)
            .bind(business)
            .bind(client_order_id)
            .map(|row: PgRow| Order {
                id: row.get("id"),
                client_order_id: row.get("client_order_id"),
                business: row.get("business"),
                state: row.get("state"),
                create_time: row.get("create_time"),
                create_by: row.get("create_by"),
                update_time: row.get("update_time"),
                update_by: row.get("update_by"),
            })
            .fetch_one(self.pool.as_ref())
            .await?;
        Ok(res)
    }

    async fn get_detail(&self, id: &Uuid) -> Result<OrderModel, Error> {
        tracing::info!("Database Execute - Order Get Detail Query");

        let mut order = sqlx::query(db_query::ORDER_GET)
            .bind(&id)
            .map(|row: PgRow| OrderModel {
                id: row.get("id"),
                client_order_id: row.get("client_order_id"),
                business: row.get("business"),
                state: row.get("state"),
                histories: vec![],
                create_time: row.get("create_time"),
                update_time: row.get("update_time"),
            })
            .fetch_one(self.pool.as_ref())
            .await?;

        sqlx::query(db_query::HISTORY_GET)
            .bind(&id)
            .map(|row: PgRow| {
                order.histories.push(HistoryModel {
                    from_state: row.get("from_state"),
                    to_state: row.get("to_state"),
                    create_time: row.get("create_time"),
                    create_by: row.get("create_by"),
                })
            })
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(order)
    }

    async fn exists_client_order_id(
        &self,
        business: &str,
        client_order_id: &str,
    ) -> Result<bool, Error> {
        tracing::info!("Database Execute - Order Exists Order Id Query");

        let res = sqlx::query(db_query::ORDER_EXISTS_BY_CLIENT_ORDER_ID)
            .bind(business)
            .bind(client_order_id)
            .map(|row: PgRow| row.get("exists"))
            .fetch_one(self.pool.as_ref())
            .await?;
        Ok(res)
    }
}

impl DbRepository {
    async fn insert_history<'ex, EX>(
        &self,
        tx: EX,
        order_id: &Uuid,
        order: &HistoryModel,
        actor: &Uuid,
    ) -> Result<(), Error>
    where
        EX: 'ex + Executor<'ex, Database = Postgres>,
    {
        tracing::info!("Database Execute - Order History Creation Query");
        let time_now = db_time_now();
        sqlx::query(db_query::HISTORY_INSERT)
            .bind(&Uuid::new_v4())
            .bind(&order_id)
            .bind(&order.from_state)
            .bind(&order.to_state)
            .bind(time_now)
            .bind(actor)
            .bind(time_now)
            .bind(actor)
            .execute(tx)
            .await?;
        Ok(())
    }
}
