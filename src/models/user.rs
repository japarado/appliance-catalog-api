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
