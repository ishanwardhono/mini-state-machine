use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub id: Uuid,
    pub code: String,
    pub url: String,
    pub create_time: chrono::NaiveDateTime,
    pub create_by: Uuid,
    pub update_time: chrono::NaiveDateTime,
    pub update_by: Uuid,
}
