use axum::{extract::State, routing::post, Json, Router};
use sqlx::PgPool;

use super::configs::{GetUserData, GetUserPostData, GetUserReturnVal};

pub async fn get_user_logic(
    State(pool): State<PgPool>,
    Json(data): Json<GetUserPostData>,
) -> GetUserReturnVal {
    // First, get the user data from db, and then check if the password matches the specified password
    let result = sqlx::query!("SELECT * FROM users WHERE password = $1", data.password)
        .fetch_one(&pool)
        .await
        .map_err(|e| GetUserReturnVal::DBERROR(e.to_string()));

    let return_data;
    // Throw a DB error if the query wasn't successful
    match result {
        Result::Ok(record) => return_data = record,
        Result::Err(err) => return err,
    }

    let pswd = data.password;
    match &return_data.password {
        // If matches, then proceed by returning the data in
        pswd => {
            let response_val = GetUserData {
                username: return_data.username,
                password: return_data.password,
            };

            return GetUserReturnVal::POST(response_val);
        }
        // If it doesn't match, then return unauthorized error code
        _ => return GetUserReturnVal::UNAUTHORIZED(String::from("Unauthorized")),
    }
}
