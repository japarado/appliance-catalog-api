#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono;
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod controllers;
mod database;
mod errors;
mod middlewares;
mod models;
mod repositories;
mod routes;
mod schema;
mod services;
mod utils;

#[derive(Clone)]
pub struct AppData {
    conn_pool: database::Pool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_data = AppData {
        conn_pool: database::create_pool(),
    };

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(app_data.clone())
            .service(index)
            .configure(routes::config)
            .default_service(web::route().to(fallback_route))
            .wrap(middlewares::auth_middleware::Logging)
            .wrap(Cors::new().finish())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(
                    env::var("COOKIE_SECRET")
                        .unwrap_or("DEFAULT_SECRET".to_string())
                        .as_bytes(),
                )
                .name("auth")
                .path("/")
                .domain(env::var("APP_DOMAIN").unwrap_or("localhost".to_string()))
                .max_age(chrono::Duration::days(1).num_seconds())
                .secure(false),
            ))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("localhost:8000")?
    };

    server.run().await
}

#[get("")]
async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("Catalog API root")
}

async fn fallback_route(_req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().json("Route not found")
}
