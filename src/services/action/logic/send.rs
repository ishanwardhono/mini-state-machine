use hyper::{Body, Client, Request};
use serde_json::json;
use std::sync::Arc;

use crate::services::{action::model::Action, client::ClientServiceLogic};

pub async fn execute(client_logic: Arc<ClientServiceLogic>, client_code: String, action: Action) {
    let client = client_logic.get_by_code(&client_code).await;

    if client.is_err() {
        tracing::error!("Error getting client: {}", client_code);
        return;
    }

    let url = client.unwrap().url;

    let body = json!(action);
    let req = Request::builder()
        .method("POST")
        .uri(url)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()));

    if req.is_err() {
        tracing::error!("State Action Got Problem");
        return;
    }

    let client = Client::new();
    let resp = client.request(req.unwrap()).await;

    if resp.is_err() {
        tracing::error!("Error on calling client");
        return;
    }

    tracing::info!("Response: {}", resp.unwrap().status());
}
