use super::{ReturnVal, RootPostBody};
use axum::{extract::Path, Json};

// returns hello world when sending a get request in /
pub async fn hello_world() -> ReturnVal {
    ReturnVal::GET(String::from("Hello world"))
}

// Returns the same message as whatever sent in the msg of the body
#[axum::debug_handler]
pub async fn print_msg(Json(data): Json<RootPostBody>) -> ReturnVal {
    let message_to_print = data.msg;

    println!("Message sent by the user::: {}", message_to_print);

    ReturnVal::GET(message_to_print)
}

#[axum::debug_handler]
pub async fn print_query_msg(Path((msg, another_message)): Path<(String, String)>) -> ReturnVal {
    println!("Message sent by the user::: {}, {}", msg, another_message);

    let msg = &msg[..];
    let another_message = &another_message[..];

    let total_str = format!("{} {}", msg, another_message);

    ReturnVal::GET(total_str)
}
