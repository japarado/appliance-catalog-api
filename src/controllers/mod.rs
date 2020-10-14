use crate::errors::ServiceError;
use actix_web::HttpResponse;

pub mod auth_controller;
pub type GenericResponse = Result<HttpResponse, ServiceError>;

pub fn ok_closure<T: serde::Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(data)
}
