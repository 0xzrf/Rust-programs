use axum::Router;
pub mod routes;

use routes::root::return_root_router;

#[tokio::main]
async fn main() {
    let router = Router::new().merge(return_root_router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
