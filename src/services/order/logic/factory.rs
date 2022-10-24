use crate::{
    cores::error::service::Error,
    services::{
        diagram::logic::factory as diagram_factory,
        order::{
            logic::{get, insert},
            model::{entity::Order, request::OrderRequest, response::OrderResponse},
            repo::db::DbRepo,
        },
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct Factory {
    pub repo: Arc<dyn DbRepo>,
    pub diagram_factory: Arc<dyn diagram_factory::Logic>,
}

#[async_trait]
pub trait Logic {
    async fn insert(&self, req: &OrderRequest, actor: &Uuid) -> Result<OrderResponse, Error>;
    async fn get(&self, id: &Uuid) -> Result<Order, Error>;
}

#[async_trait]
impl Logic for Factory {
    async fn insert(&self, req: &OrderRequest, actor: &Uuid) -> Result<OrderResponse, Error> {
        tracing::info!("Logic Execute - Insert Order");
        insert::execute(self.repo.clone(), self.diagram_factory.clone(), req, actor).await
    }

    async fn get(&self, id: &Uuid) -> Result<Order, Error> {
        tracing::info!("Logic Execute - Insert Order");
        get::execute(self.repo.clone(), id).await
    }
}
