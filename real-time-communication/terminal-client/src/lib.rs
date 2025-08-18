mod communication;
mod errors;
mod helper;
mod user_onboard;

use tokio::runtime::Runtime;
pub use {communication::*, errors::MainErrors, user_onboard::print_welcome_message};

pub fn run() -> Result<(), MainErrors> {
    print_welcome_message();
    let user_name = std::env::var("USER").unwrap();

    let mut communication = Communication::build(user_name);

    block_on(communication.user_response_onboarding()).unwrap();

    Ok(())
}

pub fn block_on<F: Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}
