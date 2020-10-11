#[macro_use]
extern crate diesel;

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;


mod controllers;
mod database;
mod routes;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .service(index)
            .configure(routes::config)
            .default_service(web::route().to(fallback_route))
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
