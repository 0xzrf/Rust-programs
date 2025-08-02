use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetUserPostData {
    pub password: String,
}

#[derive(Serialize)]
pub struct GetUserData {
    pub username: String,
    pub password: String,
}

pub enum GetUserReturnVal {
    POST(GetUserData), // The string passed will be the success message
    DBERROR(String),
    UNAUTHORIZED(String),
}

impl IntoResponse for GetUserReturnVal {
    fn into_response(self) -> axum::response::Response {
        match self {
            GetUserReturnVal::POST(success_msg) => {
                (StatusCode::OK, Json(success_msg)).into_response()
            }
            GetUserReturnVal::DBERROR(db_error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(db_error)).into_response()
            }
            GetUserReturnVal::UNAUTHORIZED(unauth_msg) => {
                (StatusCode::FORBIDDEN, Json(unauth_msg)).into_response()
            }
        }
    }
}
