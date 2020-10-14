use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error as DBError};

#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "Internal Server error")]
    InternalServerError,

    #[fail(display = "Bad Request: {}", _0)]
    BadRequest(String),

    #[fail(display = "Unauthorized")]
    Unauthorized,

    #[fail(display = "Conflict: {}", _0)]
    Conflict(String),

    #[fail(display = "Not Found")]
    NotFound,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::Conflict(ref message) => HttpResponse::Conflict().json(message),
            ServiceError::NotFound => HttpResponse::NotFound().json("Not Found"),
        }
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> Self {
        match error {
            DBError::NotFound => ServiceError::NotFound,
            _ => ServiceError::InternalServerError,
        }
    }
}
