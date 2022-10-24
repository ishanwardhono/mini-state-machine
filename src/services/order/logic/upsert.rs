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
    order: &'a OrderRequest,
    actor: &'a Uuid,
) -> Result<OrderResponse, Error> {
    let result = logic.insert(order, actor).await;
    if result.is_ok() {
        return Ok(result?);
    }

    let err = result.unwrap_err();
    if err != Error::BadRequest("Order already exists".to_owned()) {
        return Err(err);
    }

    let result = logic
        .state_update(
            &OrderStateUpdateRequest {
                id: None,
                order_id: order.order_id.clone(),
                business: Some(order.business.clone()),
                state: order.state.clone(),
            },
            actor,
        )
        .await?;

    Ok(result)
}
