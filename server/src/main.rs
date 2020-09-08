use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::Filter;

//mod db;
//use db::connect;

mod access_token;
mod authorize;
mod signin;
mod signup;

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

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
        .or(signup::routes(pool.clone()))
        .or(signin::routes(pool.clone()))
        .or(authorize::routes())
        .or(access_token::routes());

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
    Ok(())
}
