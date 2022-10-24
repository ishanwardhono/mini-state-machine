use crate::{
    cores::error::service::Error,
    services::{
        diagram::logic::factory as diagram_factory,
        order::{
            model::{entity::Order, request::OrderStateUpdateRequest},
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
) -> Result<(), Error> {
    tracing::debug!("executing ...");
    let curr_order = validate(repo.clone(), order).await?;
    diagram_factory
        .valid_transition(&curr_order.business, &curr_order.state, &order.state)
        .await?;
    repo.state_update(&curr_order.id, &order.state, actor).await
}

async fn validate(repo: Arc<dyn DbRepo>, order: &OrderStateUpdateRequest) -> Result<Order, Error> {
    if order.id.is_none() {
        let mut validation = validation::Fields::new();
        if order.business.is_none() || order.order_id.is_none() {
            validation.add_str("id is empty, then business and order_id are required");
        }
        validation.check()?;

        return Ok(repo
            .get_by_order_id(
                &order.business.as_ref().unwrap(),
                &order.order_id.as_ref().unwrap(),
            )
            .await?);
    }

    Ok(repo.get(order.id.as_ref().unwrap()).await?)
}
