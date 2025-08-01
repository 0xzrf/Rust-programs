use axum::{http::StatusCode, response::IntoResponse, Json};

pub enum GetReturnVal {
    Get(String),
    Error,
}

impl IntoResponse for GetReturnVal {
    fn into_response(self) -> axum::response::Response {
        match self {
            GetReturnVal::Get(data) => (StatusCode::OK, Json(data)).into_response(),
            GetReturnVal::Error => (StatusCode::BAD_REQUEST).into_response(),
        }
    }
}
