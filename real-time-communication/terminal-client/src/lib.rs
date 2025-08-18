mod communication;
mod errors;
mod helper;
mod user_onboard;

use tokio::runtime::Runtime;
pub use {communication::*, errors::MainErrors, user_onboard::print_minimal_welcome};

pub fn run() -> Result<(), MainErrors> {
    print_minimal_welcome();
    let user_name = std::env::var("USER").unwrap();

    let mut communication = Communication::build(user_name);

    block_on(communication.user_response_onboarding()).unwrap();

    Ok(())
}

pub fn block_on<F: Future>(future: F) -> F::Output {
    let rt = Runtime::new().unwrap();
    rt.block_on(future)
}
