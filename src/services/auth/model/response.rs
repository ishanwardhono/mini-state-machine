use serde_derive::Serialize;

#[derive(Serialize)]
pub struct UserCreateResponse {
    pub username: String,
}

#[derive(Serialize)]
pub struct UserKeyResponse {
    pub username: String,
    pub key: String,
}
