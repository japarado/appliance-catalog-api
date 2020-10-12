use crate::schema::*;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
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
