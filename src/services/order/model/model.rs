use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct OrderModel {
    pub id: Uuid,
    pub client_order_id: String,
    pub business: String,
    pub state: String,
    pub histories: Vec<HistoryModel>,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct HistoryModel {
    pub from_state: String,
    pub to_state: String,
    pub create_time: chrono::NaiveDateTime,
    pub create_by: Uuid,
}
