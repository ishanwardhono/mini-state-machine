use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    InternalServerError,
    BadRequest(String),
}

impl Error {
    pub fn new(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Error::BadRequest(e.to_string()),
            _ => Error::InternalServerError,
        }
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}
