use crate::{
    cores::error::service::Error,
    services::{
        action::ActionServiceLogic,
        diagram::DiagramServiceLogic,
        order::{
            logic::{get, insert, state_update, upsert},
            model::{
                model::OrderModel,
                request::{OrderRequest, OrderStateUpdateRequest},
                response::OrderResponse,
            },
            repo::db::DbRepo,
        },
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct Factory {
    pub repo: Arc<dyn DbRepo>,
    pub diagram_logic: Arc<DiagramServiceLogic>,
    pub action_logic: Arc<ActionServiceLogic>,
}

#[async_trait]
pub trait Logic {
    async fn insert(&self, req: OrderRequest, actor: &Uuid) -> Result<OrderResponse, Error>;
    async fn upsert(
        &self,
        req: OrderStateUpdateRequest,
        actor: &Uuid,
    ) -> Result<OrderResponse, Error>;
    async fn state_update(
        &self,
        req: OrderStateUpdateRequest,
        actor: &Uuid,
    ) -> Result<OrderResponse, Error>;
    async fn get_detail(&self, business: &str, client_order_id: &str) -> Result<OrderModel, Error>;
}

#[async_trait]
impl Logic for Factory {
    async fn insert(&self, req: OrderRequest, actor: &Uuid) -> Result<OrderResponse, Error> {
        tracing::info!("Logic Execute - Insert Order");
        insert::execute(
            self.repo.clone(),
            self.diagram_logic.clone(),
            self.action_logic.clone(),
            req,
            actor,
        )
        .await
    }

    async fn upsert(
        &self,
        req: OrderStateUpdateRequest,
        actor: &Uuid,
    ) -> Result<OrderResponse, Error> {
        tracing::info!("Logic Execute - Insert Order");
        upsert::execute(self, req, actor).await
    }

    async fn state_update(
        &self,
        req: OrderStateUpdateRequest,
        actor: &Uuid,
    ) -> Result<OrderResponse, Error> {
        tracing::info!("Logic Execute - State Update Order");
        state_update::execute(
            self.repo.clone(),
            self.diagram_logic.clone(),
            self.action_logic.clone(),
            req,
            actor,
        )
        .await
    }

    async fn get_detail(&self, business: &str, client_order_id: &str) -> Result<OrderModel, Error> {
        tracing::info!("Logic Execute - Get Order");
        get::execute(self.repo.clone(), business, client_order_id).await
    }
}
