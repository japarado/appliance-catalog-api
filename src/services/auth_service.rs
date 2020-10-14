use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::user::{AuthUser, LoginUser, NewUser, SlimUser, User};
use crate::models::{Multiple, Single};
use crate::repositories::user_repository;
use crate::utils;
use crate::AppData;
use actix_identity::Identity;
use actix_web::HttpRequest;

pub fn register(conn: &Conn, user: NewUser) -> Single<User> {
    match user_repository::find_by_email(conn, &user.email) {
        Ok(_) => Err(ServiceError::Conflict("User already exists".into())),
        Err(_) => {
            let new_user: NewUser = NewUser {
                email: user.email,
                password: utils::create_hash(&user.password),
                display_name: user.display_name,
                profile_picture: user.profile_picture,
                bio: user.bio,
            };
            Ok(user_repository::store(conn, new_user)?)
        }
    }
}

pub fn login(conn: &Conn, user: LoginUser) -> Single<User> {
    match user_repository::find_by_email(conn, &user.email) {
        Ok(found_user) => {
            if utils::verify_hash(&user.password, &found_user.password) {
                Ok(found_user)
            } else {
                Err(ServiceError::Unauthorized)
            }
        }
        Err(error) => Err(error),
    }
}

pub fn generate_jwt(user: AuthUser) -> Result<String, ServiceError> {
    Ok(utils::create_token(&user)?)
}
