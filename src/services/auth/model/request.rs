use crate::cores::auth::Role;
use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UserCreateRequest {
    pub username: String,
    pub role: Role,
}
