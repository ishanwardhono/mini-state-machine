use serde::Serialize;

#[derive(Serialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub business: String,
    pub state: String,
}
