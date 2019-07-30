extern crate actix_web;
use actix_web::{HttpServer, App, HttpRequest, web, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    "Hello Person! id:?"
}

fn hello(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/{id}/{name}/", web::get().to(hello))
    }).bind("0.0.0.0:8080")
        .unwrap()
        .run()
}
