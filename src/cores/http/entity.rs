use serde::Serialize;

pub type User = i32;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}
