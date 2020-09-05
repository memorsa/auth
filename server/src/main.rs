use serde::Deserialize;
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

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

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

    // let static_files = warp::path("static").and(warp::fs::dir("./client/public"));
    // let routes = static_files.or(authorize::routes()).or(access_token::routes());

    let db = Arc::new(Mutex::new(HashMap::<String, User>::new()));
    let db = warp::any().map(move || Arc::clone(&db));

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and(db.clone())
        .and_then(register);
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(db.clone())
        .and_then(login);

    let logout = warp::path("logout").map(|| "Hello from logout");
    let api = warp::path("api").and(register.or(login).or(logout));

    let counter_db = Arc::new(Mutex::new(0));
    let counter_db = warp::any().map(move || Arc::clone(&counter_db));
    let counter = warp::path("counter")
        .and(counter_db.clone())
        .and_then(counter);

    let routes = authorize::routes()
        .or(access_token::routes())
        .or(api)
        .or(counter);

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

async fn counter(db: Arc<Mutex<u8>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut counter = db.lock().await;
    *counter += 1;
    Ok(counter.to_string())
}

async fn register(
    new_user: User,
    db: Arc<Mutex<HashMap<String, User>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut users = db.lock().await;
    if users.contains_key(&new_user.username) {
        return Ok(StatusCode::BAD_REQUEST);
    }
    users.insert(new_user.username.clone(), new_user);
    Ok(StatusCode::CREATED)
}

async fn login(
    credentials: User,
    db: Arc<Mutex<HashMap<String, User>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let users = db.lock().await;
    match users.get(&credentials.username) {
        None => Ok(StatusCode::BAD_REQUEST),
        Some(user) => {
            if credentials.password == user.password {
                Ok(StatusCode::OK)
            } else {
                Ok(StatusCode::UNAUTHORIZED)
            }
        }
    }
}
