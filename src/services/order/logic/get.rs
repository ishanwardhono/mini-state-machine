use crate::{
    cores::error::service::Error,
    services::order::{model::model::OrderModel, repo::db::DbRepo},
    utils::validation,
};
use std::sync::Arc;

pub async fn execute<'a>(
    repo: Arc<dyn DbRepo>,
    business: &'a str,
    client_order_id: &'a str,
) -> Result<OrderModel, Error> {
    tracing::debug!("executing ...");
    validate(business, client_order_id)?;
    repo.get_detail(business, client_order_id).await
}

fn validate(business: &str, client_order_id: &str) -> Result<(), Error> {
    let mut validation = validation::Fields::new();
    if business.is_empty() {
        validation.add_str("business is empty");
    }
    if client_order_id.is_empty() {
        validation.add_str("Order ID is empty");
    }
    validation.check()
}
