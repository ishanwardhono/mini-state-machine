use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Deserialize, Serialize, Clone, Copy)]
#[sqlx(type_name = "role")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Admin,
    BusinessClient,
}

impl Role {
    fn default() -> Self {
        Role::BusinessClient
    }

    fn valid(&self, role: Self) -> bool {
        if self.level() <= role.level() {
            return true;
        }
        false
    }

    fn level(&self) -> u8 {
        match self {
            Role::Admin => 1,
            Role::BusinessClient => 2,
        }
    }
}
