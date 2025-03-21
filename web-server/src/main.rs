use std::{
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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
