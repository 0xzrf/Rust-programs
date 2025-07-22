use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RootPostBody {
    pub msg: String,
}

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
