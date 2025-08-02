use axum::{
    routing::{get, post},
    Router,
};
pub mod routes;
use routes::{create_contact, create_user, get_user, root};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await.unwrap();

    let router = Router::new()
        .route("/", get(root::root_logic::return_hello_world))
        .route("/create-user", post(create_user::logic::set_user_logic))
        .route("/get-user", post(get_user::logic::get_user_logic))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
