use actix_web::{delete, get, post, HttpRequest, HttpResponse, Responder};
use std::env;

#[post("/login")]
pub async fn login(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("Login")
}

#[post("/register")]
pub async fn register(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("Register")
}

#[delete("/logout")]
pub async fn logout(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("Logout")
}

use argon2::Config;
fn create_hash(text: &String) -> String {
    let text_to_hash = text.to_owned().into_bytes();
    let salt = env::var("SALT")
        .unwrap_or(String::from("Default salt value"))
        .into_bytes();

    let config = Config::default();
    argon2::hash_encoded(&text_to_hash, &salt, &config).unwrap()
}

fn verify_hash(text: String, hash: String) -> bool {
    return argon2::verify_encoded(&hash, &text.into_bytes()).unwrap();
}
