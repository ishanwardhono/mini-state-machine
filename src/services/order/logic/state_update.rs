use crate::{
    cores::error::service::Error,
    services::{
        diagram::logic::factory as diagram_factory,
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
    diagram_factory: Arc<dyn diagram_factory::Logic>,
    order: &'a OrderStateUpdateRequest,
    actor: &'a Uuid,
) -> Result<OrderResponse, Error> {
    tracing::debug!("executing ...");
    validate(order)?;
    let curr_order = validate_order(repo.clone(), order).await?;
    diagram_factory
        .valid_transition(&curr_order.business, &curr_order.state, &order.state)
        .await?;
    repo.state_update(&curr_order.id, &order.state, actor)
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

    validation.check()
}

async fn validate_order(
    repo: Arc<dyn DbRepo>,
    order: &OrderStateUpdateRequest,
) -> Result<Order, Error> {
    if order.id.is_none() {
        let mut validation = validation::Fields::new();
        if order.business.is_none() || order.client_order_id.is_none() {
            validation.add_str("id is empty, then business and client_order_id are required");
        }
        validation.check()?;

        return Ok(repo
            .get_by_client_order_id(
                &order.business.as_ref().unwrap(),
                &order.client_order_id.as_ref().unwrap(),
            )
            .await?);
    }

    Ok(repo.get(order.id.as_ref().unwrap()).await?)
}
