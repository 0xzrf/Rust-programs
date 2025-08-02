use super::super::get_contacts::configs::UserContact;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetContactBody {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct GetContactResponse {
    pub success: bool,
    pub msg: String,
    pub contacts: UserContact,
}

pub enum GetContactResponseStatus {
    POST(GetContactResponse), // The string passed will be the success message
    UNAUTHORIZED,
    DBERROR(String),
}

impl IntoResponse for GetContactResponseStatus {
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
