use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

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
    let template = "<!DOCTYPE html>
                    <html>
                      <head>
                        <title>Warp Handlebars template example</title>
                      </head>
                      <body>
                        <h1>Hello {{user}}!</h1>
                      </body>
                    </html>";

    let mut hb = Handlebars::new();
    // register the template
    hb.register_template_string("template.html", template)
        .unwrap();
    let hb = Arc::new(hb);
    //let body = warp::reply::html("Authorize!");
    let body = render(
        WithTemplate {
            name: "template.html",
            value: json!({"user" : "Warp"}),
        },
        hb.clone(),
    );
    Ok(warp::reply::with_status(body, StatusCode::OK))
}

async fn post_authorize() -> Result<impl warp::Reply, warp::Rejection> {
    let body = warp::reply::html("Authorized!");
    Ok(warp::reply::with_status(body, StatusCode::OK))
}
