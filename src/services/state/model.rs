
use crate::cores::database::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct State {
    pub id: i32,
    pub code: String,
    pub description: Option<String>,
    pub webhooks: Option<Vec<String>>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "states"]
pub struct NewState<'a> {
    pub code: &'a str,
    pub description: &'a str,
    pub webhooks: Vec<&'a str>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStateRequest {
    pub code: String,
    pub description: String,
    pub webhooks: Vec<String>,
}