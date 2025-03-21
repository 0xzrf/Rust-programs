use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}
};

fn main() {

    let listener = match TcpListener::bind("127.0.0.1:3000") {
        Ok(val) => val,
        Err(e) => panic!("Unable to initilize the server: {e}")
    };

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handle_connection(stream); 
    }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request= buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (status_line, file_name) = if http_request == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}
