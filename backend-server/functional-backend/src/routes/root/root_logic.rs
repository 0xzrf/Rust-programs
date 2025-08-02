use super::root_configs::GetReturnVal;

#[axum::debug_handler]
pub async fn return_hello_world() -> GetReturnVal {
    GetReturnVal::Get(String::from("Welcome back!"))
}
