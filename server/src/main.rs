use std::env;
use tokio_postgres::NoTls;
use warp::Filter;

mod db;

use db::connect;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse()
        .unwrap();

    const DB_URL: &str = "postgres://ryzfreof:qiwHjU94MS5jOkkDVoChIg-m5-3_8NZO@rajje.db.elephantsql.com:5432/ryzfreof";

    let client = connect(DB_URL).await.unwrap();

    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world!"])
        .await
        .unwrap();

    let value: String = rows[0].get(0);

    // Match any request and return hello world!
    let routes = warp::any().map(move || format!("{}", value));

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
