use actix::prelude::*;
use actix_web::dev::Body;
use actix_web::http::{header::CONTENT_TYPE, header::SERVER, HeaderValue, StatusCode};
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use askama::Template;
use ring::rand::{SecureRandom, SystemRandom};
use serde::Deserialize;
use std::env;

mod db;

use db::{PgConnection, RandomWorld};

struct TheClient {
    id: String,
    name: String,
    redirect_uri: String,
}

#[derive(Deserialize)]
struct AuthorizeForm {
    client_id: String,
    state: String,
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

async fn get_authorize(query: web::Query<AuthorizeInfo>) -> impl Responder {
    // TODO: use middleware
    // check login status, redirect to login if not logged in

    // Get client id from query
    let client_id = &query.client_id;
    let client_name = env::var("client_name").unwrap_or_else(|_| String::from("Test"));

    // find the client, mocking the finding process here
    // if client found, render page
    // else render 404
    if let Ok(id) = env::var("client_id") {
        if &id == client_id {
            let s = AuthorizeTemplate {
                name: &client_name,
                text: "Welcome!",
                client_id: client_id,
                response_type: &query.response_type[..],
                scope: match &query.scope {
                    Some(value) => Some(&value[..]),
                    None => None,
                },
            }
            .render()
            .unwrap();

            return HttpResponse::Ok().content_type("text/html").body(s);
        }
    }

    HttpResponse::NotFound().body("404")
}

async fn post_authorize(form: web::Form<AuthorizeForm>) -> impl Responder {
    // TODO: use middleware
    // check login status, redirect to login if not logged in

    // find the client
    let client = TheClient {
        name: String::from("test"),
        id: String::from("test"),
        redirect_uri: String::from("https://google.com"),
    };
    // if found, generate code and make a redirect to redirect url
    // else render 404

    // TODO: associate code with client & current user
    let mut code = vec![0; 8];
    SystemRandom::new().fill(code.as_mut_slice()).unwrap();

    let location = format!(
        "{}?code={}&state={}",
        client.redirect_uri,
        hex::encode(code),
        form.state
    );
    HttpResponse::Found().header("location", location).finish()
}

async fn token() -> impl Responder {
    "abcde"
}

async fn db_actor_test(db: web::Data<Addr<PgConnection>>) -> Result<HttpResponse, Error> {
    let res = db.send(RandomWorld).await.unwrap();
    match res {
        Ok(body) => {
            let mut res = HttpResponse::with_body(StatusCode::OK, Body::Bytes(body));
            res.headers_mut()
                .insert(SERVER, HeaderValue::from_static("Actix"));
            res.headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
            Ok(res)
        }
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    }
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));

    const DB_URL: &str = "postgres://ryzfreof:qiwHjU94MS5jOkkDVoChIg-m5-3_8NZO@rajje.db.elephantsql.com:5432/ryzfreof";

    HttpServer::new(move || {
        App::new()
            .data_factory(|| db::PgConnection::connect(DB_URL))
            .route("/", web::get().to(index))
            .route("/authorize", web::get().to(get_authorize))
            .service(web::resource("/authorize").route(web::post().to(post_authorize)))
            .route("/token", web::post().to(token))
            .route("/db_actor_test", web::get().to(db_actor_test))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
