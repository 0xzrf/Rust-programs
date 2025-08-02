use super::configs::*;
use axum::extract::State;
use sqlx::{types::Json, PgPool};

pub async fn create_contact_logic(
    State(pool): State<PgPool>,
    Json(data): Json<CreateContactBody>,
) -> CreateContactResponseStatus {
    let result = sqlx::query!("SELECT * FROM users WHERE password = $1", data.password)
        .fetch_one(&pool)
        .await;

    let db_return_data;
    // Throw a DB error if the query wasn't successful
    match result {
        Result::Ok(record) => db_return_data = record,
        Result::Err(_) => return CreateContactResponseStatus::UNAUTHORIZED,
    }

    let result = sqlx::query!(
        "INSERT INTO contacts (email, phone, username) VALUES ($1, $2, $3)",
        data.email,
        data.phone,
        db_return_data.username
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
