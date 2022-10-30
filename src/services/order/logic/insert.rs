use crate::{
    cores::error::service::Error,
    services::{
        diagram::DiagramServiceLogic,
        order::{
            model::{request::OrderRequest, response::OrderResponse},
            repo::db::DbRepo,
        },
    },
    utils::validation,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    diagram_logic: Arc<DiagramServiceLogic>,
    order: &'a OrderRequest,
    actor: &'a Uuid,
) -> Result<OrderResponse, Error> {
    tracing::debug!("executing ...");
    validate(order)?;
    validate_order_data(repo.clone(), order).await?;
    diagram_logic
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
    if let Some(client_order_id) = order.client_order_id.as_ref() {
        if repo
            .exists_client_order_id(&order.business, client_order_id)
            .await?
        {
            return Err(Error::BadRequest("Order already exists".to_owned()));
        }
    }
    Ok(())
}
