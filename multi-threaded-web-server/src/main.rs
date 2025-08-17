use std::net::TcpListener;

use multi_threaded_web_server::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.run_thread_pooling(stream);
    }
    println!("Shutting down the server");
}
