use crate::{
    cores::error::service::Error,
    services::order::{model::model::OrderModel, repo::db::DbRepo},
    utils::validation,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn execute(repo: Arc<dyn DbRepo>, id: &Uuid) -> Result<OrderModel, Error> {
    tracing::debug!("executing ...");
    validate(id)?;
    repo.get_detail(id).await
}

fn validate(id: &Uuid) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if id.is_nil() {
        validation.add_str("id is empty");
    }
    validation.check()
}
