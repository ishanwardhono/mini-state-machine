use serde::{Deserialize, Serialize};

use crate::cores::auth::role::Role;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: Role,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
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
