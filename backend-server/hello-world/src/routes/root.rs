use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};

pub enum ReturnVal {
    GET(String),
    Error,
}

impl IntoResponse for ReturnVal {
    fn into_response(self) -> Response {
        match self {
            ReturnVal::GET(data) => (StatusCode::OK, Json(data)).into_response(),
            ReturnVal::Error => (StatusCode::NOT_FOUND).into_response(),
        }
    }
}

pub async fn hello_world() -> ReturnVal {
    ReturnVal::GET(String::from("Hello world"))
}
