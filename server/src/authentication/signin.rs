use super::password_helper::authenticate;
use askama::Template;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use warp::{filters::BoxedFilter, http::Uri, Filter, Rejection, Reply};

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Template)]
#[template(path = "signin.html")]
struct SigninTemplate;

async fn signin(pool: PgPool, signin: User) -> Result<impl Reply, Rejection> {
    let rec = sqlx::query_file!("queries/find-user-by-name.sql", signin.username,)
        .fetch_one(&pool)
        .await
        .unwrap();

    if authenticate(&rec.password_digest, signin.password.as_bytes()) {
        Ok(warp::redirect(Uri::from_static("/")))
    } else {
        // TODO: build a response with 403
        Ok(warp::redirect(Uri::from_static("/signin")))
    }
}

pub fn routes(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    let path = warp::path("signin");
    let get = path.and(warp::get()).map(|| SigninTemplate);
    let post = path
        .and(warp::post())
        .and(warp::any().map(move || pool.clone()))
        .and(warp::body::form())
        .and_then(signin);

    get.or(post).boxed()
}
