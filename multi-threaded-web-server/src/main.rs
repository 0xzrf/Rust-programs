use multi_threaded_web_server::STAsyncIO;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (mut stream, addr) = listener.accept().await.unwrap();
        println!("New connection from: {addr}");

        // spawn a task to handle this connection
        let st_async_io = STAsyncIO;

        st_async_io.execute(stream).await;
    }

    // Code to execute thread pool
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // let pool = ThreadPool::new(5);

    // for stream in listener.incoming().take(2) {
    //     let stream = stream.unwrap();

    //     pool.run_thread_pooling(stream);
    // }
    // println!("Shutting down the server");

    println!("Shutting down the server");
}
