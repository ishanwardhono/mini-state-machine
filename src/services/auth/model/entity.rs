use super::role::Role;

pub struct User {
    pub id: i32,
    pub username: String,
    pub role: Role,
    pub create_time: chrono::NaiveDateTime,
    pub update_time: chrono::NaiveDateTime,
}
