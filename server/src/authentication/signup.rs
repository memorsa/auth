use super::password_helper::hash;
use askama::Template;
use cookie::{Cookie, CookieJar, Key};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::{Seeder, SipHasher};
use serde::Deserialize;
use sqlx::postgres::PgPool;
use std::env;
use warp::{
    filters::BoxedFilter,
    http::{header, Uri},
    Filter, Rejection, Reply,
};

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate;

async fn signup(pool: PgPool, new_user: User) -> Result<impl Reply, Rejection> {
    let rec = sqlx::query_file!(
        "queries/create-user.sql",
        new_user.username,
        hash(new_user.password.as_bytes())
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let redirect = warp::redirect(Uri::from_static("/"));
    Ok(warp::reply::with_header(
        redirect,
        header::SET_COOKIE,
        session(rec.id.to_string()),
    ))
}

pub fn routes(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    let path = warp::path("signup");
    let get = path.and(warp::get()).map(|| SignupTemplate);
    let post = path
        .and(warp::post())
        .and(warp::any().map(move || pool.clone()))
        .and(warp::body::form())
        .and_then(signup);

    get.or(post).boxed()
}

fn session(user_id: String) -> String {
    let mut jar = CookieJar::new();

    let secret_key_base = env::var("SECRET_KEY_BASE").unwrap();
    let mut rng: Pcg64 = Seeder::from(secret_key_base).make_rng();
    let mut master_key = [0u8; 32];
    rng.fill_bytes(&mut master_key);
    let key = Key::derive_from(&master_key);

    let new_cookie = Cookie::build("user_id", user_id)
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    jar.private(&key).add(new_cookie);
    let cookie_header: Vec<String> = jar.iter().map(|cookie| cookie.to_string()).collect();

    cookie_header.join("; ")
}
