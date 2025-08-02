use axum::{extract::State, Json};
use sqlx::PgPool;

use super::configs::{GetUserData, GetUserPostData, GetUserReturnVal};

pub async fn get_user_logic(
    State(pool): State<PgPool>,
    Json(data): Json<GetUserPostData>,
) -> GetUserReturnVal {
    // First, get the user data from db, and then check if the password matches the specified password
    let result = sqlx::query!("SELECT * FROM users WHERE password = $1", data.password)
        .fetch_one(&pool)
        .await;

    let return_data;
    // Throw a DB error if the query wasn't successful
    match result {
        Result::Ok(record) => return_data = record,
        Result::Err(_) => return GetUserReturnVal::UNAUTHORIZED(String::from("Unauthorized call")),
    }
    let response_val = GetUserData {
        username: return_data.username,
        password: return_data.password,
    };

    return GetUserReturnVal::POST(response_val);
}
