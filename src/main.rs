extern crate actix_web;
use actix_web::{http, server, App, HttpRequest, Path, Responder};

fn index(_req: HttpRequest) -> impl Responder {
    "Hello Person! id:?"
}

fn hello(info: Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn main() {
    server::new(|| {
        App::new()
            .route("/", http::Method::GET, index)
            .route("/{id}/{name}/", http::Method::GET, hello)
    }).bind("0.0.0.0:80")
        .unwrap()
        .run();
}
