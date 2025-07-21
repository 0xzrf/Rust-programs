use axum::{routing::get, Router};

pub mod routes;

pub use routes::root::hello_world;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(hello_world));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
