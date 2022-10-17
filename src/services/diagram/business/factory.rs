use std::sync::Arc;

use crate::cores::error::service::Error;
use async_trait::async_trait;

use super::insert;

pub struct BusinessFactory {}

impl BusinessFactory {
    pub fn new() -> Arc<dyn Business> {
        Arc::new(Self {})
    }
}

#[async_trait]
pub trait Business {
    async fn insert(&self) -> Result<(), Error>;
}

#[async_trait]
impl Business for BusinessFactory {
    async fn insert(&self) -> Result<(), Error> {
        insert::execute().await
    }
}
