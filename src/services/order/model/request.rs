use serde::Deserialize;

#[derive(Deserialize)]
pub struct OrderRequest {
    pub order_id: Option<String>,
    pub business: String,
    pub state: String,
}
