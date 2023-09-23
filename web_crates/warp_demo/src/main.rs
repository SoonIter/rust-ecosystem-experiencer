use std::{collections::HashMap, env, time::Duration};

use warp::{filters::cookie, Filter};

const WEB_DIR: &str = "web/";

async fn delay_2s() {
  tokio::time::sleep(Duration::from_millis(2000)).await
}
async fn say_hello(name: String) -> Result<impl warp::Reply, warp::Rejection> {
  delay_2s().await;
  Ok(format!("Hello {}", name))
}

async fn get_items(
  name: String,
  query: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
  delay_2s().await;
  Ok(format!("{}{:#?}", name, query))
}

// use cargo watch -x "run" to auto-reloading web server during development
// cargo watch -x "run"
// cargo watch -x "run --example basic"
#[tokio::main]
async fn main() -> Result<(), ()> {
  env::set_var("RUST_APP_LOG", "DEBUG");

  pretty_env_logger::init_custom_env("RUST_APP_LOG");

  let log = warp::log("basic");

  // async  macro-path
  let hello = warp::path!("hello" / String).and_then(say_hello);

  // sync  macro-path
  let hi = warp::path!("hi" / String).map(|name| format!("Hi {}", name));

  // async path
  let items = warp::get()
    .and(warp::path("items"))
    .and(warp::path::param::<String>())
    .and(warp::query::<HashMap<String, String>>())
    .and(warp::path::end())
    .and_then(get_items);

  // static web server
  let dir_static = warp::fs::dir(WEB_DIR);

  let apis = hello.or(hi).or(items).with(log);

  warp::serve(apis.or(dir_static)).run(([0, 0, 0, 0], 3000)).await;

  Ok(())
}
