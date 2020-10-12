use crate::database::Conn;
use crate::errors::ServiceError;
use crate::models::user::{NewUser, User};
use crate::models::{Multiple, Single};
use crate::repositories::user_repository;
use crate::utils;

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

pub fn login(conn: &Conn, user: User) -> Single<User> {
    match user_repository::find_by_email(conn, &user.email) {
        Ok(found_user) => {
            Ok(found_user)
        }
        Err(error) => Err(error)
    }
}
