use crate::schema::*;

#[derive(Debug, Identifiable, Queryable)]
pub struct User {
    id: i32,
    email: String,
    password: String,
    display_name: Option<String>,
    profile_picture: Option<String>,
    bio: Option<String>
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    email: String,
    password: String,
    display_name: Option<String>,
    profile_picture: Option<String>,
    bio: Option<String>
}
