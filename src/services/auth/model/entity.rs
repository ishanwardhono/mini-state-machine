use crate::cores::auth::Role;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub role: Role,
    pub create_time: chrono::NaiveDateTime,
    pub create_by: Uuid,
    pub update_time: chrono::NaiveDateTime,
    pub update_by: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct Claim {
    pub sub: String,
    pub exp: usize,
    pub jti: Option<String>,
    pub nbf: Option<usize>,
    pub iat: Option<usize>,
    pub iss: Option<String>,
    pub aud: Option<String>,
}
