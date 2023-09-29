use net_demo::ThreadPool;
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }

  println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
  let mut buf_reader = BufReader::new(&mut stream);
  let buffer = buf_reader.fill_buf().unwrap();
  let get = b"GET / HTTP/1.1";
  let sleep = b"GET /sleep HTTP/1.1";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", "./web/hello.html")
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    ("HTTP/1.1 200 OK", "./web/hello.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND", "./web/404.html")
  };

  let contents = fs::read_to_string(filename).unwrap();

  let response = format!(
    "{}\r\nContent-Length: {}\r\n\r\n{}",
    status_line,
    contents.len(),
    contents
  );

  stream.write_all(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
