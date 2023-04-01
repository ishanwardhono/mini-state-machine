use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Action {
    pub from_state: String,
    pub to_state: String,
    pub business: String,
    pub order_id: String,
    pub action_time: chrono::NaiveDateTime,
}

pub struct InsertRetryAction {
    pub client: String,
    pub from_state: String,
    pub to_state: String,
    pub business: String,
    pub order_id: String,
    pub action_time: chrono::NaiveDateTime,
}
