use super::configs::{CreateUserPostBody, PostReturnVal};
use axum::{extract::State, routing::post, Json, Router};
use dotenvy::dotenv;
use sqlx::PgPool;

pub async fn set_user_logic(
    State(pool): State<PgPool>,
    Json(data): Json<CreateUserPostBody>,
) -> PostReturnVal {
    let result = sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2)",
        data.username,
        data.password
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => PostReturnVal::POST(String::from("User created")),
        Err(err) => {
            eprintln!("DB error: {}", err);
            PostReturnVal::DBERROR(String::from("Internal server error"))
        }
    }
}
