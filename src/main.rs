use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .default_service(web::route().to(fallback_route))
    })
    .bind("localhost:8000")?
    .run()
    .await
}

#[get("")]
async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("Catalog API root")
}

async fn fallback_route(_req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().json("Route not found")
}
