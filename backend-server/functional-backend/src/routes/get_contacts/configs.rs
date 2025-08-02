use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetContactsBody {
    pub password: String,
}

#[derive(Serialize)]
pub struct GetContactsResponse {
    pub success: bool,
    pub msg: String,
    pub contacts: Vec<String>,
}

pub enum GetContactsResponseStatus {
    POST(GetContactsResponse), // The string passed will be the success message
    UNAUTHORIZED,
    DBERROR(String),
}

impl IntoResponse for GetContactsResponseStatus {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::POST(data) => (StatusCode::ACCEPTED, Json(data)).into_response(),
            Self::UNAUTHORIZED => {
                (StatusCode::FORBIDDEN, String::from("Unauthorized")).into_response()
            }
            Self::DBERROR(db_error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, db_error).into_response()
            }
        }
    }
}
