use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(greet))
                .route("/hello/{name}", web::get().to(greet))
                .route("/health_check", web::get().to(health_check))
        })
        .listen(listener)?
        .run();

    Ok(server)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

// we can remove req if unused thanks to actix_web
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}