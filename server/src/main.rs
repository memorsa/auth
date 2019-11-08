use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use askama::Template;
use ring::rand::{SecureRandom, SystemRandom};
use serde::Deserialize;
use std::env;

struct Client {
    id: String,
    name: String,
    redirect_uri: String,
}

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

fn get_authorize(query: web::Query<AuthorizeInfo>) -> impl Responder {
    // find the client
    // if client found, render page
    // else render 404
    let s = AuthorizeTemplate {
        name: "Everyone",
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

fn post_authorize() -> impl Responder {
    // TODO: extract form data first

    // check login status, redirect to login if not logged in

    // find the client
    let client = Client {
        name: String::from("test"),
        id: String::from("test"),
        redirect_uri: String::from("https://google.com"),
    };
    // if found, generate code and make a redirect to redirect url
    // else render 404

    // TODO: associate code with client & current user
    let mut code = vec![0; 8];
    SystemRandom::new().fill(code.as_mut_slice()).unwrap();

    let location = format!("{}?code={}", client.redirect_uri, hex::encode(code));
    HttpResponse::Found().header("location", location).finish()
}

fn token() -> impl Responder {
    "abcde"
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .route("/authorize", web::get().to(get_authorize))
            .service(web::resource("/authorize").route(web::post().to(post_authorize)))
            .route("/token", web::post().to(token))
    })
    .bind(format!("0.0.0.0:{}", port))
    .unwrap()
    .run()
    .unwrap();
}
