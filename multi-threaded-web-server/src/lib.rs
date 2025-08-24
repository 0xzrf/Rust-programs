use std::{
    fs,
    io::{BufReader, prelude::*},
    net::TcpStream,
    thread,
    time::Duration,
};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader as async_BufReader},
    net::TcpStream as async_TcpStream,
};

mod mt_async_io;
mod st_async_io;
mod thread_pooling;
pub use st_async_io::*;
pub use thread_pooling::*;

pub fn handle_connection(mut stream: TcpStream) {
    let stream_buffer = BufReader::new(&stream);
    let http_buffer = stream_buffer.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &http_buffer[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}

pub async fn async_handle_connection(mut stream: async_TcpStream) {
    let (read_half, mut write_half) = stream.split();
    let stream_buffer = async_BufReader::new(read_half);
    let http_buffer = stream_buffer.lines().next_line().await.unwrap();

    let (status_line, filename) = match &http_buffer.unwrap()[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    write_half.write_all(response.as_bytes()).await.unwrap();
}
