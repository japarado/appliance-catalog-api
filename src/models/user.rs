use crate::schema::*;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { email: user.email }
    }
}

impl From<web::Json<NewUser>> for NewUser {
    fn from(new_user: web::Json<NewUser>) -> Self {
        Self {
            email: new_user.email.clone(),
            password: new_user.password.clone(),
            display_name: new_user.display_name.clone(),
            profile_picture: new_user.profile_picture.clone(),
            bio: new_user.bio.clone(),
        }
    }
}

impl From<web::Json<LoginUser>> for LoginUser {
    fn from(login_user: web::Json<LoginUser>) -> Self {
        Self {
            email: login_user.email.clone(),
            password: login_user.password.clone()
        }
    }
}

// Type alias for logged in user
