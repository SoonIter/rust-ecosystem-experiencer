use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

fn new_sharded_db(num_shards: usize) -> ShardedDb {
  let mut db = Vec::with_capacity(num_shards);
  for _ in 0..num_shards {
    db.push(Mutex::new(HashMap::new()));
  }
  Arc::new(db)
}

fn hash(key: String) -> usize {
  key.as_bytes().iter().map(|&ch| ch as usize).sum()
}

#[tokio::main]
async fn main() {
  let addr = "127.0.0.1:6379";
  let listener = TcpListener::bind(addr).await.unwrap();

  println!("Tcp Listening!");
  let db = new_sharded_db(10);

  loop {
    // 第二个被忽略的项中包含有新连接的 `IP` 和端口信息
    let (socket, _) = listener.accept().await.unwrap();
    println!("Accepted!");
    let db = db.clone();
    tokio::spawn(async move {
      process(socket, db).await;
    });
  }
}

async fn process(socket: TcpStream, db: ShardedDb) {
  use mini_redis::Command::{self, Get, Set};

  // `mini-redis` 提供的便利函数，使用返回的 `connection` 可以用于从 socket 中读取数据并解析为数据帧
  let mut connection = Connection::new(socket);

  // 使用 `read_frame` 方法从连接获取一个数据帧：一条redis命令 + 相应的数据
  while let Some(frame) = connection.read_frame().await.unwrap() {
    let response = match Command::from_frame(frame).unwrap() {
      Set(cmd) => {
        let key = cmd.key().to_string();
        let value = cmd.value().clone();
        let mut db = db
          .get(hash(key.clone()) % db.len())
          .expect("no db found by hash")
          .lock()
          .unwrap();
        db.insert(key, value);
        Frame::Simple("OK".to_string())
      }
      Get(cmd) => {
        let key = cmd.key().to_string();
        let db = db[hash(key.clone()) % db.len()].lock().unwrap();
        if let Some(value) = db.get(key.as_str()) {
          Frame::Bulk(value.clone())
        } else {
          Frame::Null
        }
      }
      cmd => panic!("unimplemented {:?}", cmd),
    };

    // 将请求响应返回给客户端
    connection.write_frame(&response).await.unwrap();
  }
}

#[test]
fn hash_should_work() {
  assert_eq!(hash("hello".to_owned()), 532);
}
