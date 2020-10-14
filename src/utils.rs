use crate::errors::ServiceError;
use crate::models::user::AuthUser;
use argon2::Config;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // issuer
    iss: String,

    // subject
    sub: String,

    // issued at
    iat: i64,

    // expiry
    exp: i64,

    // user email
    id: i32,
    email: String,
    display_name: Option<String>,
    profile_picture: Option<String>,
    bio: Option<String>,
}

// struct to get converted from token and back
impl Claims {
    fn with_user(user: &AuthUser) -> Self {
        Claims {
            iss: "localhost".into(),
            sub: "auth".into(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
            id: user.id,
            email: user.email.clone(),
            display_name: user.display_name.clone(),
            profile_picture: user.profile_picture.clone(),
            bio: user.bio.clone(),
        }
    }
}

impl From<Claims> for AuthUser {
    fn from(claims: Claims) -> Self {
        AuthUser {
            id: claims.id,
            email: claims.email,
            display_name: claims.display_name,
            profile_picture: claims.profile_picture,
            bio: claims.bio,
        }
    }
}

pub fn create_token(data: &AuthUser) -> Result<String, ServiceError> {
    let claims = Claims::with_user(data);
    let encoding_key = jsonwebtoken::EncodingKey::from_secret(get_secret().as_bytes());
    encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_err| ServiceError::InternalServerError)
}

pub fn decode_token(token: &str) -> Result<AuthUser, ServiceError> {
    let secret = get_secret();
    let secret_as_bytes = secret.as_bytes();
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(secret_as_bytes);
    decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| Ok(data.claims.into()))
        .map_err(|_err| ServiceError::Unauthorized)?
}

pub fn get_secret() -> String {
    env::var("JWT_SECRET").unwrap_or("default jwt secret".into())
}

pub fn create_hash(text: &String) -> String {
    let text_to_hash = text.to_owned().into_bytes();
    let salt = env::var("SALT")
        .unwrap_or(String::from("Default salt value"))
        .into_bytes();

    let config = Config::default();
    argon2::hash_encoded(&text_to_hash, &salt, &config).unwrap()
}

pub fn verify_hash(text: &String, hash: &String) -> bool {
    let cloned_text = text.clone();
    return argon2::verify_encoded(hash, &cloned_text.into_bytes()).unwrap();
}
