use warp::http::StatusCode;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("access_token"))
        .and_then(access_token)
}

async fn access_token() -> Result<impl warp::Reply, warp::Rejection> {
    let body = warp::reply::html("token");
    Ok(warp::reply::with_status(body, StatusCode::OK))
}
