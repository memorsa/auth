use serde::Deserialize;
use sqlx::postgres::PgPool;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

async fn signup(pool: PgPool, new_user: User) -> Result<impl Reply, Rejection> {
    let rec = sqlx::query!(
        r#"
INSERT INTO users (name, password_digest)
VALUES ( $1, $2 )
RETURNING id
        "#,
        new_user.username,
        new_user.password
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    Ok(rec.id.to_string())
}

pub fn routes(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path("signup"))
        .and(warp::any().map(move || pool.clone()))
        .and(warp::body::json())
        .and_then(signup)
        .boxed()
}
