use axum::{routing::get, Router};

pub mod routes;

pub use routes::root::hello_world;

fn main() {
    let router = Router::<&str>::new().route("/", get(hello_world));
}
