extern crate actix_web;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use serde::Deserialize;
use std::env;

#[derive(Template)]
#[template(path = "authorize.html")]
struct AuthorizeTemplate<'a> {
    name: &'a str,
    text: &'a str,
    client_id: &'a str,
    response_type: &'a str,
    scope: Option<&'a str>,
}

#[derive(Deserialize)]
struct AuthorizeInfo {
    response_type: String,
    client_id: String,
    scope: Option<String>,
}

fn authorize(query: web::Query<AuthorizeInfo>) -> impl Responder {
    let s = AuthorizeTemplate {
        name: "Guys",
        text: "Welcome!",
        client_id: &query.client_id[..],
        response_type: &query.response_type[..],
        scope: match &query.scope {
            Some(value) => Some(&value[..]),
            None => None,
        },
    }
    .render()
    .unwrap();

    HttpResponse::Ok().content_type("text/html").body(s)
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/authorize", web::get().to(authorize))
    })
    .bind(format!("0.0.0.0:{}", port))
    .unwrap()
    .run()
    .unwrap();
}
