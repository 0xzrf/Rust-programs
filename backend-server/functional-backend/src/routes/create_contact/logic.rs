use super::configs::*;
use crate::routes::helper::verify_user;
use axum::extract::State;
use sqlx::{types::Json, PgPool};

pub async fn create_contact_logic(
    State(pool): State<PgPool>,
    Json(data): Json<CreateContactBody>,
) -> CreateContactResponseStatus {
    let verified = verify_user(State(pool.clone()), data.password).await;

    if verified.is_err() {
        return CreateContactResponseStatus::UNAUTHORIZED;
    }

    let result = sqlx::query!(
        "INSERT INTO contacts (email, phone, username) VALUES ($1, $2, $3)",
        data.email,
        data.phone,
        verified.unwrap().username
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            let response = CreateContactResponse {
                msg: String::from("Contact created successfully"),
                success: true,
            };
            return CreateContactResponseStatus::POST(response);
        }
        Err(err) => return CreateContactResponseStatus::DBERROR(err.to_string()),
    }
}
