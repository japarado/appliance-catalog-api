use actix_web::{error::ResponseError, HttpResponse};

#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "Internal Server error")]
    InternalServerError,

    #[fail(display = "Bad Request: {}", _0)]
    BadRequest(String),

    #[fail(display = "Unauthorized")]
    Unauthorized,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            },
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized")
        }
    }
}
