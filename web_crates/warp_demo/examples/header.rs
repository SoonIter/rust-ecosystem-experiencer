#![deny(warnings)]
use std::{env, net::SocketAddr};
use warp::Filter;

/// Create a server that requires header conditions:
///
/// - `Host` is a `SocketAddr`
/// - `Accept` is exactly `*/*`
///
/// If these conditions don't match, a 404 is returned.
#[tokio::main]
async fn main() {
  env::set_var("RUST_APP_LOG", "DEBUG");

  pretty_env_logger::init_custom_env("RUST_APP_LOG");

  let log = warp::log("basic");

  // For this example, we assume no DNS was used,
  // so the Host header should be an address.
  let host = warp::header::<SocketAddr>("host");

  // Match when we get `accept: */*` exactly.
  let accept_stars = warp::header::exact("accept", "*/*");

  let routes = host
    .and(accept_stars)
    .map(|addr| format!("accepting stars on {}", addr))
    .with(log);

  warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}

// curl 0.0.0.0:3000 -H 'host: 127.0.0.1:3000' -H 'Accept:*/*' -X POST
