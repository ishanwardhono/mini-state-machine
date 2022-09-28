use super::role::Role;

#[derive(Clone)]
pub struct UserCreateRequest {
    pub username: String,
    pub role: Role,
}
