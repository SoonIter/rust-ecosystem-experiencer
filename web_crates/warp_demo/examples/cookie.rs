use std::env;
use warp::{
  http::{HeaderValue, Response, StatusCode},
  Filter,
};

/// If these conditions don't match, a 404 custom error is returned.
#[tokio::main]
async fn main() {
  env::set_var("RUST_APP_LOG", "DEBUG");

  pretty_env_logger::init_custom_env("RUST_APP_LOG");

  let log = warp::log("basic");

  let login_path = warp::get().and(warp::path!("login")).map(|| {
    let mut login_success_response = Response::new("Login successfully!!");
    let headers = login_success_response.headers_mut();
    headers.append(
      "Set-Cookie",
      HeaderValue::from_static("world=123ABC;"),
    );
    headers.append(
      "Set-Cookie",
      HeaderValue::from_static("hello=ABC123;"),
    );
    headers.append("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    login_success_response
  });

  let routes = login_path
    .or(warp::any().map(|| {
      let mut error_404_response = Response::new("404 Page not found !!!! QAQ");
      *error_404_response.status_mut() = StatusCode::NOT_FOUND;
      error_404_response
    }))
    .with(log);

  warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}

// curl 0.0.0.0:3000 -H 'host: 127.0.0.1:3000/login' -X GET
