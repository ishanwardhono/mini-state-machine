use super::http::entity::ErrorResponse;
use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display, PartialEq)]
pub enum Error {
    #[display(fmt = "Please try again later!")]
    InternalError(String),

    #[display(fmt = "Bad Request")]
    BadRequest(String),

    #[display(fmt = "No Data")]
    NotFound(String),
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
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let mut message = match self {
            Self::InternalError(message) => message.to_string(),
            Self::BadRequest(message) => message.to_string(),
            _ => "".to_string(),
        };

        let error_msg: String;
        if message != "" {
            error_msg = format!("{} : {}", self.status_code().to_string(), self.to_string());
        } else {
            error_msg = self.status_code().to_string();
            message = self.to_string();
        }
        let error_response = ErrorResponse {
            error: error_msg,
            message: message,
        };

        HttpResponse::build(self.status_code()).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}
