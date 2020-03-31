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
    let body = warp::reply::html("Authorize!");
    Ok(warp::reply::with_status(body, StatusCode::OK))
}

async fn post_authorize() -> Result<impl warp::Reply, warp::Rejection> {
    let body = warp::reply::html("Authorized!");
    Ok(warp::reply::with_status(body, StatusCode::OK))
}
