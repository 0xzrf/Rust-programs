use axum::{http::StatusCode, response::IntoResponse, Json};

pub enum PostReturnVal {
    POST(String), // The string passed will be the success message
    UNAUTHORIZED(String),
}
