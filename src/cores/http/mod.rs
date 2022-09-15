use serde::Serialize;

pub mod middleware;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}
