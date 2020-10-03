use super::password_helper::hash;
use super::session::session;
use askama::Template;
use serde::Deserialize;
use sqlx::postgres::PgPool;
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
