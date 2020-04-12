use handlebars::Handlebars;
use serde_json::json;
use warp::http::StatusCode;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get().or(post())
}

fn get() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("authorize"))
        .and_then(get_authorize)
}

fn post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("authorize"))
        .and_then(post_authorize)
}

async fn get_authorize() -> Result<impl warp::Reply, warp::Rejection> {
    let mut hb = Handlebars::new();
    hb.register_template_file("authorize", "./server/templates/authorize.html")
        .unwrap();
    let render = hb
        .render(
            "authorize",
            &json!({
                "name": "Warp",
                "client_id": "xxxx",
                "response_type": "authorization",
                "scope": "abcd"
            }),
        )
        .unwrap_or_else(|err| err.to_string());
    let body = warp::reply::html(render);
    Ok(warp::reply::with_status(body, StatusCode::OK))
}

async fn post_authorize() -> Result<impl warp::Reply, warp::Rejection> {
    let body = warp::reply::html("Authorized!");
    Ok(warp::reply::with_status(body, StatusCode::OK))
}
