use serde::Deserialize;
use sqlx::postgres::PgPool;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

async fn signin(pool: PgPool, user: User) -> Result<impl Reply, Rejection> {
    let rec = sqlx::query!(
        r#"
SELECT * FROM users
WHERE name = $1 AND password_digest = $2
        "#,
        user.username,
        user.password
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    Ok(rec.name.to_string())
}

pub fn routes(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path("signin"))
        .and(warp::any().map(move || pool.clone()))
        .and(warp::body::json())
        .and_then(signin)
        .boxed()
}
