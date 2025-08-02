use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateContactBody {
    pub email: String,
    pub phone: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateContactResponse {
    pub success: bool,
    pub msg: String,
}

pub enum CreateContactResponseStatus {
    POST(CreateContactResponse), // The string passed will be the success message
    UNAUTHORIZED,
    DBERROR(String),
}

impl IntoResponse for CreateContactResponseStatus {
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
