extern crate actix_web;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));

    HttpServer::new(move || App::new().route("/authorize", web::get().to(index)))
        .bind(format!("127.0.0.1:{}", port))
        .unwrap()
        .run()
        .unwrap();
}
