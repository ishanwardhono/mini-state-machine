use uuid::Uuid;

#[allow(dead_code)]
pub struct Order {
    pub id: Uuid,
    pub order_id: String,
    pub business: String,
    pub state: String,
    pub create_time: chrono::NaiveDateTime,
    pub create_by: Uuid,
    pub update_time: chrono::NaiveDateTime,
    pub update_by: Uuid,
}
