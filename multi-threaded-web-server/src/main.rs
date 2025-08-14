use std::net::TcpListener;

use multi_threaded_web_server::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(1);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.run_thread_pooling(stream);
    }
}
