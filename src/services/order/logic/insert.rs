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
    validate_order_data(repo.clone(), order).await?;
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

async fn validate_order_data(repo: Arc<dyn DbRepo>, order: &OrderRequest) -> Result<(), Error> {
    if let Some(order_id) = order.order_id.as_ref() {
        if repo.exists_order_id(&order.business, order_id).await? {
            return Err(Error::BadRequest("Order already exists".to_owned()));
        }
    }
    Ok(())
}
