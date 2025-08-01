use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserPostBody {
    pub username: String,
    pub password: String,
}

pub enum PostReturnVal {
    POST(String), // The string passed will be the success message
    DBERROR(String),
}

impl IntoResponse for PostReturnVal {
    fn into_response(self) -> axum::response::Response {
        match self {
            PostReturnVal::POST(success_msg) => (StatusCode::OK, Json(success_msg)).into_response(),
            PostReturnVal::DBERROR(db_error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(db_error)).into_response()
            }
        }
    }
}
