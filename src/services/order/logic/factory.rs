use crate::{
    cores::error::service::Error,
    services::{
        diagram::logic::factory as diagram_factory,
        order::{
            logic::{get, insert},
            model::{entity::Order, request::OrderRequest},
            repo::db::DbRepo,
        },
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

impl LogicFactory {
    pub fn new(
        repo: Arc<dyn DbRepo>,
        diagram_factory: Arc<dyn diagram_factory::Logic>,
    ) -> Arc<dyn Logic> {
        Arc::new(Self {
            repo,
            diagram_factory,
        })
    }
}

pub struct LogicFactory {
    repo: Arc<dyn DbRepo>,
    diagram_factory: Arc<dyn diagram_factory::Logic>,
}

#[async_trait]
pub trait Logic {
    async fn insert(&self, req: &OrderRequest, actor: &Uuid) -> Result<(), Error>;
    async fn get(&self, id: &Uuid) -> Result<Order, Error>;
}

#[async_trait]
impl Logic for LogicFactory {
    async fn insert(&self, req: &OrderRequest, actor: &Uuid) -> Result<(), Error> {
        tracing::info!("Logic Execute - Insert Order");
        insert::execute(self.repo.clone(), self.diagram_factory.clone(), req, actor).await
    }

    async fn get(&self, id: &Uuid) -> Result<Order, Error> {
        tracing::info!("Logic Execute - Insert Order");
        get::execute(self.repo.clone(), id).await
    }
}
