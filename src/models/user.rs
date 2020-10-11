use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    id: i32,
    email: String,
    password: String,
    display_name: Option<String>,
    profile_picture: Option<String>,
    bio: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    email: String,
    password: String,
    display_name: Option<String>,
    profile_picture: Option<String>,
    bio: Option<String>,
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
