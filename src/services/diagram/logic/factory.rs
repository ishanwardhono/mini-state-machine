use std::sync::Arc;

use crate::cores::error::service::Error;
use async_trait::async_trait;

use super::insert;

pub struct LogicFactory {}

impl LogicFactory {
    pub fn new() -> Arc<dyn Logic> {
        Arc::new(Self {})
    }
}

#[async_trait]
pub trait Logic {
    async fn insert(&self) -> Result<(), Error>;
}

#[async_trait]
impl Logic for LogicFactory {
    async fn insert(&self) -> Result<(), Error> {
        insert::execute().await
    }
}
