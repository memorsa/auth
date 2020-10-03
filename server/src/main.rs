use sqlx::postgres::PgPoolOptions;
use std::env;
use warp::Filter;

mod access_token;
mod authentication;
mod authorize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse()
        .unwrap();

    const DB_URL: &str = "postgres://ryzfreof:qiwHjU94MS5jOkkDVoChIg-m5-3_8NZO@rajje.db.elephantsql.com:5432/ryzfreof";
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(DB_URL)
        .await?;

    let static_files = warp::path("static").and(warp::fs::dir("./client/public"));

    let routes = static_files
        .or(authentication::signup::routes(pool.clone()))
        .or(authentication::signin::routes(pool.clone()))
        .or(authorize::routes())
        .or(access_token::routes());

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
    Ok(())
}
