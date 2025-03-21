use std::net::TcpListener;


fn main() {

    let listener = match TcpListener::bind("127.0.0.1:3000") {
        Ok(val) => val,
        Err(e) => panic!("Unable to initilize the server: {e}")
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
    }

}
