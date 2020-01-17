use std::env;
use tokio_postgres::NoTls;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse()
        .unwrap();

    const DB_URL: &str = "postgres://ryzfreof:qiwHjU94MS5jOkkDVoChIg-m5-3_8NZO@rajje.db.elephantsql.com:5432/ryzfreof";

    let (client, connection) = tokio_postgres::connect(DB_URL, NoTls).await.unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world!"])
        .await
        .unwrap();

    let value: String = rows[0].get(0);

    // Match any request and return hello world!
    let routes = warp::any().map(move || format!("{}", value));

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
