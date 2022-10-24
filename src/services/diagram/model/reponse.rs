use serde::Serialize;

#[derive(Serialize)]
pub struct CreatedResponse {
    pub business: String,
}
