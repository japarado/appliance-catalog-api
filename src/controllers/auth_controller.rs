use actix_web::{delete, get, post, HttpRequest, HttpResponse, Responder};

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
