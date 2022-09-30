use std::fmt::Display;

pub const DBERROR_VIOLATE_UNIQUE: &str = "23505";

pub enum AuthError {
    NotProvided,
    InvalidFormat,
    UnsupportedType,
    NotPermitted(String),
    InvalidUser(String),
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NotProvided => "Auth Token not provided".to_owned(),
            Self::InvalidFormat => "Invalid Authorization Format".to_owned(),
            Self::UnsupportedType => "Unsupported Authorization Type".to_owned(),
            Self::NotPermitted(user) => format!("User {} not permitted", user).to_owned(),
            Self::InvalidUser(user) => format!("Invalid User {}", user).to_owned(),
        };

        write!(f, "{}", msg)
    }
}
