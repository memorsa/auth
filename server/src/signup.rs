use askama::Template;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate<'a> {
    name: &'a str,
}

async fn signup(pool: PgPool, new_user: User) -> Result<impl Reply, Rejection> {
    let rec = sqlx::query_file!(
        "queries/create-user.sql",
        new_user.username,
        new_user.password
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    Ok(rec.id.to_string())
}

pub fn routes(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    let get = warp::get().map(|| SignupTemplate { name: "Askama" });
    let post = warp::post()
        .and(warp::path("signup"))
        .and(warp::any().map(move || pool.clone()))
        .and(warp::body::json())
        .and_then(signup);

    get.or(post).boxed()
}
