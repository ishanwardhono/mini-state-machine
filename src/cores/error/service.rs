use super::types::DBERROR_VIOLATE_UNIQUE;
use crate::cores::http::entity::ErrorResponse;
use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;
use std::fmt::Display;

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

    pub fn unauth_from<E>(e: E) -> Error
    where
        E: Display,
    {
        Error::Unauthorized(e.to_string())
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
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::from_db(e)
    }
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: self.status_code().to_string(),
            message: self.get_message(),
        };
        tracing::error!("{}: {}", error_response.error, error_response.message);
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
