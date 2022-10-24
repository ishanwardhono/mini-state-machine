use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct OrderRequest {
    pub client_order_id: Option<String>,
    pub business: String,
    pub state: String,
}

#[derive(Deserialize)]
pub struct OrderStateUpdateRequest {
    pub id: Option<Uuid>,
    pub client_order_id: Option<String>,
    pub business: Option<String>,
    pub state: String,
}
