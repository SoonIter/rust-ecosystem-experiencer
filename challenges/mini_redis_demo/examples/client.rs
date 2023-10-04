use bytes::Bytes;
use mini_redis::cmd::{Get, Set};
use mini_redis::{client, Command};
use std::io::Write;
use std::thread;
use std::time::SystemTime;
use tokio::sync::mpsc;

fn prompt(name: &str) -> String {
  let mut line = String::new();
  print!("{}", name);
  std::io::stdout().flush().unwrap();
  std::io::stdin()
    .read_line(&mut line)
    .expect("Error: Could not read a line");

  line.trim().to_string()
}

#[tokio::main]
async fn main() {
  let (tx, mut rx) = mpsc::channel(32);

  let manager = tokio::spawn(async move {
    // Establish a connection to the server
    // 建立到 redis 服务器的连接
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // 开始接收消息
    while let Some(cmd) = rx.recv().await {
      match cmd {
        Command::Get(get_cmd) => {
          let key = get_cmd.key();
          let x = client.get(key).await;
          println!("\n{:?}", x.unwrap().unwrap());
        }
        Command::Set(set_cmd) => {
          let key = set_cmd.key();
          let val = set_cmd.value().to_owned();
          client.set(key, val).await.unwrap();
        }
        _ => {}
      }
    }
  });

  thread::spawn(move || async {
    manager.await.unwrap();
  });
  loop {
    let input = prompt("> ");

    if input == "now" {
      let unixtime = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
      print!("Current Unix time is {:?}\n", unixtime);
    } else if input == "exit" {
      break;
    } else if input.starts_with("SET") {
      let v: Vec<String> = input.split(' ').map(|s| s.to_owned()).collect();
      let key = &v[1];
      let value = &v[2];
      tx.send(Command::Set(Set::new(
        key,
        Bytes::from(value.to_owned()),
        None,
      )))
      .await
      .expect("set error");

      println!("set successfully")
    } else if input.starts_with("GET") {
      let v: Vec<String> = input.split(' ').map(|s| s.to_owned()).collect();
      let key = &v[1];
      tx.send(Command::Get(Get::new(key)))
        .await
        .expect("get error");

      println!("get successfully")
    }
  }
}
