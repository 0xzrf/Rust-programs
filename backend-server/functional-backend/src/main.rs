use axum::Router;
pub mod routes;

use routes::{create_contact, root, set_user};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .merge(root::return_root_router())
        .merge(set_user::logic::set_user_router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
