use axum::extract::State;
use sqlx::PgPool;

pub struct UserData {
    pub username: String,
    pub pwd: String,
}
pub async fn verify_user(State(pool): State<&PgPool>, pwd: &String) -> Result<UserData, ()> {
    let result = sqlx::query!("SELECT * FROM users WHERE password = $1", pwd)
        .fetch_one(pool)
        .await;

    // Throw a DB error if the query wasn't successful
    match result {
        Result::Ok(data) => {
            let return_data = UserData {
                username: data.username,
                pwd: data.password,
            };
            return Ok(return_data);
        }
        Result::Err(_) => return Err(()),
    }
}
