use super::configs::GetReturnVal;
use axum::{routing::get, Router};

#[axum::debug_handler]
pub async fn return_hello_world() -> GetReturnVal {
    GetReturnVal::Get(String::from("Welcome back!"))
}

pub fn return_root_router() -> Router {
    Router::new().route("/", get(return_hello_world))
}
