use crate::{
    cores::error::service::Error,
    services::{
        action::{
            model::{Action, InsertRetryAction},
            repo::db::DbRepo,
        },
        client::ClientServiceLogic,
    },
};
use hyper::{Body, Client, Request};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub async fn execute(
    repo: Arc<dyn DbRepo>,
    client_logic: Arc<ClientServiceLogic>,
    client_code: String,
    action: Action,
    actor: &Uuid,
) -> Result<(), Error> {
    let client = client_logic.get_by_code(&client_code).await;
    let url = client?.url;

    let body = json!(action);
    let req = Request::builder()
        .method("POST")
        .uri(url)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()));

    let client = Client::new();
    let resp = client.request(req?).await;

    if resp.is_err() {
        tracing::error!(
            "Failed to execute action to {}, err: {}",
            client_code,
            resp.as_ref().unwrap_err()
        );
        let insert_retry = repo
            .insert(
                InsertRetryAction {
                    client: client_code.clone(),
                    business: action.business.clone(),
                    order_id: action.order_id.clone(),
                    from_state: action.from_state.clone(),
                    to_state: action.to_state.clone(),
                },
                actor,
            )
            .await;
        if insert_retry.is_err() {
            tracing::error!(
                "Failed to insert to retry action on {}, err: {}, data: {:?}",
                client_code,
                insert_retry.unwrap_err(),
                action
            )
        }
        let _ = resp?;
    }
    tracing::info!(
        "Successfully execute action to {} for order {} on business {}",
        client_code,
        action.order_id,
        action.business
    );
    Ok(())
}
