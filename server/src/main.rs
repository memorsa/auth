use std::env;
use warp::Filter;

mod db;
use db::connect;

mod access_token;
mod authorize;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse()
        .unwrap();

    // const DB_URL: &str = "postgres://ryzfreof:qiwHjU94MS5jOkkDVoChIg-m5-3_8NZO@rajje.db.elephantsql.com:5432/ryzfreof";

    // let client = connect(DB_URL).await.unwrap();

    // let rows = client
    //     .query("SELECT $1::TEXT", &[&"hello wrap!"])
    //     .await
    //     .unwrap();

    // let value: String = rows[0].get(0);

    let routes = authorize::routes().or(access_token::routes());

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
