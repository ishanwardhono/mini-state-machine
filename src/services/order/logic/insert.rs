use crate::{
    cores::error::service::Error,
    services::{
        diagram::logic::factory as diagram_factory,
        order::{model::request::OrderRequest, repo::db::DbRepo},
    },
    utils::validation,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    diagram_factory: Arc<dyn diagram_factory::Logic>,
    order: &'a OrderRequest,
    actor: &'a Uuid,
) -> Result<(), Error> {
    tracing::debug!("executing ...");
    validate(order)?;
    diagram_factory
        .valid_creation(&order.business, &order.state)
        .await?;
    repo.insert(order, actor).await
}

fn validate(order: &OrderRequest) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if order.business.is_empty() {
        validation.add_str("Business Code is empty");
    }
    if order.state.is_empty() {
        validation.add_str("State is empty");
    }

    validation.check()
}
