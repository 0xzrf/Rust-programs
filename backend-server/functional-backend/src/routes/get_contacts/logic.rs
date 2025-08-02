use axum::{extract::State, Json};
use sqlx::PgPool;

use super::configs::*;
use crate::routes::helper::verify_user;
pub async fn get_contacts_logic(
    State(pool): State<PgPool>,
    Json(data): Json<GetContactsBody>,
) -> GetContactsResponseStatus {
    let verified = verify_user(State(&pool), &data.password).await;

    if verified.is_err() {
        return GetContactsResponseStatus::UNAUTHORIZED;
    }

    let result = sqlx::query!(
        "SELECT * FROM contacts WHERE username = $1",
        verified.unwrap().username
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(data) => {
            let new_data = data
                .iter()
                .map(|d| UserContact {
                    email: d.email.clone(),
                    phone: d.phone.clone().unwrap(),
                })
                .collect();

            let return_val = GetContactsResponse {
                success: true,
                msg: String::from("Successfully fetched the user contacts"),
                contacts: new_data,
            };

            return GetContactsResponseStatus::POST(return_val);
        }
        Err(err) => return GetContactsResponseStatus::DBERROR(err.to_string()),
    }
}
