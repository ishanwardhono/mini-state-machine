use crate::cores::http::entity::ErrorResponse;
use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display, PartialEq)]
pub enum Error {
    #[display(fmt = "Internal Server Error")]
    InternalError(String),

    #[display(fmt = "Bad Request")]
    BadRequest(String),

    #[display(fmt = "No Data")]
    NotFound(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized(String),
}

const DBERROR_VIOLATE_UNIQUE: &str = "23505";

impl Error {
    pub fn from_db(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Error::NotFound("".to_owned()),
            sqlx::Error::Database(err) => match err.code().unwrap().to_string().as_str() {
                DBERROR_VIOLATE_UNIQUE => Error::BadRequest(err.to_string()),
                _ => Error::InternalError(err.to_string()),
            },
            _ => Error::InternalError(e.to_string()),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Error::InternalError(msg)
            | Error::BadRequest(msg)
            | Error::NotFound(msg)
            | Error::Unauthorized(msg) => msg,
        }
        .to_owned()
    }

    pub fn to_message_display(&self) -> String {
        let msg = self.get_message();
        if msg.is_empty() {
            return format!("{}", self.to_string());
        }
        format!("{}: {}", self.to_string(), msg)
    }
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: self.status_code().to_string(),
            message: self.get_message(),
        };
        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
