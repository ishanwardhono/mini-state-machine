use crate::{
    cores::{database::pg::db_time_now, error::service::Error},
    services::{
        action::{model::Action, ActionServiceLogic},
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

pub async fn execute(
    repo: Arc<dyn DbRepo>,
    diagram_logic: Arc<DiagramServiceLogic>,
    action_logic: Arc<ActionServiceLogic>,
    order: OrderStateUpdateRequest,
    actor: &Uuid,
) -> Result<OrderResponse, Error> {
    tracing::debug!("executing ...");
    validate(&order)?;
    let curr_order = validate_order(repo.clone(), &order).await?;
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
    action_logic
        .run(
            Action {
                from_state: curr_order.state,
                to_state: order.state.clone(),
                business: curr_order.business.clone(),
                order_id: curr_order.client_order_id.clone(),
                action_time: db_time_now(),
            },
            actor,
        )
        .await?;

    Ok(OrderResponse {
        id: curr_order.id,
        client_order_id: curr_order.client_order_id,
        business: curr_order.business,
        state: order.state,
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
