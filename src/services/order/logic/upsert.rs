use super::factory::Logic;
use crate::{
    cores::error::service::Error,
    services::order::model::{
        request::{OrderRequest, OrderStateUpdateRequest},
        response::OrderResponse,
    },
};
use uuid::Uuid;

pub async fn execute<'a>(
    logic: &'a impl Logic,
    order: &'a OrderStateUpdateRequest,
    actor: &'a Uuid,
) -> Result<OrderResponse, Error> {
    let result = logic
        .insert(
            &OrderRequest {
                client_order_id: Some(order.client_order_id.clone()),
                business: order.business.clone(),
                state: order.state.clone(),
            },
            actor,
        )
        .await;
    if result.is_ok() {
        return Ok(result?);
    }

    let err = result.unwrap_err();
    if err != Error::BadRequest("Order already exists".to_owned()) {
        return Err(err);
    }

    let result = logic.state_update(order, actor).await?;

    Ok(result)
}
