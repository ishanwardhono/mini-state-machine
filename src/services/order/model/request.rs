use serde::Deserialize;

#[derive(Deserialize)]
pub struct OrderRequest {
    pub client_order_id: Option<String>,
    pub business: String,
    pub state: String,
}

#[derive(Deserialize)]
pub struct OrderStateUpdateRequest {
    pub client_order_id: String,
    pub business: String,
    pub state: String,
}
