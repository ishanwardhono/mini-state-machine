use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Error {
    #[display(fmt = "Internal Server Error: Please try again later!")]
    InternalError,

    #[display(fmt = "Bad Request")]
    BadRequest(String),

    #[display(fmt = "Data Not Found")]
    NotFound(String),
}

impl Error {
    pub fn from_db(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Error::NotFound("".to_owned()),
            _ => Error::InternalError,
        }
    }
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let message = match self {
            Self::BadRequest(message) | Self::BadRequest(message) => self.to_string() + message,
            _ => self.to_string(),
        };
        HttpResponse::build(self.status_code()).json(message)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}
