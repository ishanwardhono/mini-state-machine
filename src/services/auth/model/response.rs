use serde_derive::Serialize;

#[derive(Serialize)]
pub struct UserCreateResponse {
    pub username: String,
}
