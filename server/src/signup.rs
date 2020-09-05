use warp::http::StatusCode;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

async fn signup() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::CREATED)
}

pub fn routes() -> BoxedFilter<(impl Reply,)> {
    warp::post()
        .and(warp::path("signup"))
        .and_then(signup)
        .boxed()
}
