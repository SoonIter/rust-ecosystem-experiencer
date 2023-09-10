use std::str::FromStr;

use anyhow::Result;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Article {
  user_id: Option<usize>,
  id: Option<usize>,
  title: String,
  body: String,
}

#[tokio::main]
async fn main() -> Result<()> {
  let client = reqwest::Client::new();
  // let client = reqwest::Client::builder()
  //   .proxy(reqwest::Proxy::all("htt://127.0.0.1:8899")?)
  //   .build()?;

  let url = Url::from_str("https://jsonplaceholder.typicode.com/posts")?;

  // get
  let article_get_list: Vec<Article> = client.get(url.clone()).send().await?.json().await?;
  dbg!(article_get_list.len());

  // get by query
  let mut get_url = url.clone();
  get_url.set_query(Some("id=1"));

  let article_get_one: Vec<Article> = client.get(get_url).send().await?.json().await?;
  dbg!(article_get_one);

  // post
  // 1. struct
  let my_article = Article {
    user_id: Some(1),
    id: None,
    title: "this is a title".to_string(),
    body: "this is body".to_string(),
  };

  let created_article: Article = client
    .post(url.clone())
    .json(&my_article)
    .send()
    .await?
    .json()
    .await?;
  dbg!(created_article);

  // 2. json
  let created_article: Article = client
    .post(url)
    .json(&serde_json::json!({
      "userId": 1,
      "title": "this is a title",
      "body": "this is a body"
    }))
    .send()
    .await?
    .json()
    .await?;
  dbg!(created_article);

  Ok(())
}
