use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const PORT: i32 = 3000;

fn main() {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let c = stream.read(&mut buffer).unwrap();

    println!("請求內容: {}", String::from_utf8_lossy(&buffer[..c]));
    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
