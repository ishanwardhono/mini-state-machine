use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[sqlx(type_name = "role")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Admin,
    BusinessClient,
}

impl Role {
    pub fn level(&self) -> u8 {
        match self {
            Role::Admin => 1,
            Role::BusinessClient => 2,
        }
    }
}
