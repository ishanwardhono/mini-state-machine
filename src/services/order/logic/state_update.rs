use crate::{
    cores::error::service::Error,
    services::{
        diagram::DiagramServiceLogic,
        order::{
            model::{entity::Order, request::OrderStateUpdateRequest, response::OrderResponse},
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
    order: &'a OrderStateUpdateRequest,
    actor: &'a Uuid,
) -> Result<OrderResponse, Error> {
    tracing::debug!("executing ...");
    validate(order)?;
    let curr_order = validate_order(repo.clone(), order).await?;
    diagram_logic
        .valid_transition(&curr_order.business, &curr_order.state, &order.state)
        .await?;
    repo.state_update(
        &curr_order.id,
        curr_order.state.as_str(),
        order.state.as_str(),
        actor,
    )
    .await?;
    Ok(OrderResponse {
        id: curr_order.id,
        client_order_id: curr_order.client_order_id,
        business: curr_order.business,
        state: order.state.clone(),
    })
}

fn validate(order: &OrderStateUpdateRequest) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if order.state.is_empty() {
        validation.add_str("State is empty");
    }
    if order.business.is_empty() {
        validation.add_str("business is empty");
    }
    if order.client_order_id.is_empty() {
        validation.add_str("Order ID is empty");
    }
    validation.check()
}

async fn validate_order(
    repo: Arc<dyn DbRepo>,
    order: &OrderStateUpdateRequest,
) -> Result<Order, Error> {
    Ok(repo
        .get_by_client_order_id(&order.business, &order.client_order_id)
        .await?)
}
