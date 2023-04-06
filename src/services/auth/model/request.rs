use crate::cores::auth::Role;

#[derive(Debug, Clone, PartialEq)]
pub struct UserCreateRequest {
    pub username: String,
    pub role: Role,
}
