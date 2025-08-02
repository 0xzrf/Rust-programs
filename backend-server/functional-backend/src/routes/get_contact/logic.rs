use axum::{extract::State, Json};
use sqlx::PgPool;

use super::configs::*;
use crate::routes::get_contacts::configs::UserContact;
use crate::routes::helper::verify_user;

pub async fn get_contacts_logic(
    State(pool): State<PgPool>,
    Json(data): Json<GetContactBody>,
) -> GetContactResponseStatus {
    let verified = verify_user(State(&pool), &data.password).await;

    if verified.is_err() {
        return GetContactResponseStatus::UNAUTHORIZED;
    }

    let result = sqlx::query!(
        "SELECT * FROM contacts WHERE username = $1 AND email = $2",
        verified.unwrap().username,
        data.email
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(data) => {
            let new_data = UserContact {
                email: data.email.clone(),
                phone: data.phone.clone().unwrap(),
            };

            let return_val = GetContactResponse {
                success: true,
                msg: String::from("Successfully fetched the user contacts"),
                contacts: new_data,
            };

            return GetContactResponseStatus::POST(return_val);
        }
        Err(err) => return GetContactResponseStatus::DBERROR(err.to_string()),
    }
}
