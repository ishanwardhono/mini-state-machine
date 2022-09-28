use super::role::Role;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: Role,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
}
