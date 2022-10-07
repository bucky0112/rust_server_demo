extern crate server;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{fs, path::Path, thread, time::Duration};
use server::ThreadPool;

const PORT: i32 = 3000;
const WORKER_SIZE: usize = 4;
fn main() {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();
    let pool = ThreadPool::new(WORKER_SIZE);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| { handle_connection(stream); });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let delay = b"GET /delay HTTP/1.1\r\n";

    let (status_line, path) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", Path::new("./src/HTML/hello.html"))
    } else if buffer.starts_with(delay) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", Path::new("./src/HTML/delay.html"))
    } else {
        ("HTTP/1.1 404 NOT FOUND", Path::new("./src/HTML/404.html"))
    };

    let contents = fs::read_to_string(path).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
