use multi_threaded_web_server::STAsyncIO;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = STAsyncIO::new(5).await;

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute_st_io();
    }
    println!("Shutting down the server");
}
