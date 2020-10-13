use crate::controllers::ok_closure;
use crate::database::Pool;
use crate::errors::ServiceError;
use crate::models::user::{NewUser, SlimUser, User, LoginUser};
use crate::models::Single;
use crate::services::auth_service;
use crate::utils;
use crate::AppData;
use actix_web::error::BlockingError;
use actix_web::{delete, post, web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;

#[post("/login")]
pub async fn login(data: web::Data<AppData>, payload: web::Json<LoginUser>, identity: Identity) -> impl Responder {
    web::block(move || -> Result<User, ServiceError> {
        let conn = data.conn_pool.get().unwrap();
        Ok(auth_service::login(&conn, payload.into())?)
    })
    .await
        .map(|data| {
            let token = auth_service::generate_jwt(data.clone().into()).unwrap();
            println!("JWT Generated: {}", token);
            identity.remember(token.clone());

            let token_decoded = utils::decode_token(&token).unwrap();

            println!("Token Decoded: {:?}", token_decoded);
            ok_closure(data)
        })
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

#[post("/register")]
pub async fn register(data: web::Data<AppData>, payload: web::Json<NewUser>) -> impl Responder {
    web::block(move || -> Result<User, ServiceError> {
        let conn = data.conn_pool.get().unwrap();
        Ok(auth_service::register(&conn, payload.into())?)
    })
    .await
    .map(|data| ok_closure(data))
    .map_err(|err| match err {
        BlockingError::Error(service_error) => service_error,
        BlockingError::Canceled => ServiceError::InternalServerError,
    })
}

#[delete("/logout")]
pub async fn logout(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("Logout")
}
