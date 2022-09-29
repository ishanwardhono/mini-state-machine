use derive_more::Display;

pub const DBERROR_VIOLATE_UNIQUE: &str = "23505";

#[derive(Display)]
pub enum AuthError {
    #[display(fmt = "Auth Token not provided")]
    NotProvided,

    #[display(fmt = "Invalid Authorization Format")]
    InvalidFormat,

    #[display(fmt = "Unsupported Authorization Type")]
    UnsupportedType,

    #[display(fmt = "User not permitted")]
    NotPermitted,

    #[display(fmt = "Invalid User")]
    InvalidUser,
}
