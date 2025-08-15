mod communication;
mod errors;
mod user_onboard;

pub use {errors::MainErrors, user_onboard::print_welcome_message};

pub fn run() -> Result<(), MainErrors> {
    print_welcome_message();
    Ok(())
}
