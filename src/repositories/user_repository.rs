use crate::database::Conn;
use crate::models::user::{NewUser, User};
use crate::models::{Multiple, Single};
use crate::schema::users;
use diesel::prelude::*;

pub fn store(conn: &Conn, user: NewUser) -> Single<User> {
    Ok(diesel::insert_into(users::table)
        .values(user)
        .get_result(conn)?)
}

pub fn show(conn: &Conn, id: i32) -> Single<User> {
    Ok(users::table.find(id).first(conn)?)
}

pub fn find_by_email(conn: &Conn, email: &String) -> Single<User> {
    Ok(users::table.filter(users::email.eq(email)).first(conn)?)
}
