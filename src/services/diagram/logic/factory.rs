use std::sync::Arc;

use crate::{
    cores::error::service::Error,
    services::diagram::{model::model::Diagram, repo::db::DbRepo},
};
use async_trait::async_trait;
use uuid::Uuid;

use super::insert;

pub struct LogicFactory {
    repo: Arc<dyn DbRepo>,
}

impl LogicFactory {
    pub fn new(repo: Arc<dyn DbRepo>) -> Arc<dyn Logic> {
        Arc::new(Self { repo })
    }
}

#[async_trait]
pub trait Logic {
    async fn insert(&self, req: &Diagram, actor: &Uuid) -> Result<(), Error>;
}

#[async_trait]
impl Logic for LogicFactory {
    async fn insert(&self, req: &Diagram, actor: &Uuid) -> Result<(), Error> {
        tracing::info!("Logic Execute - Insert Diagram");
        insert::execute(self.repo.clone(), req, actor).await
    }
}
