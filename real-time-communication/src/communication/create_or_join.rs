use crate::errors::OnboardErrors;
use std::{env, io};

pub fn user_response_onboarding() -> Result<(), OnboardErrors> {
    let user_name = env::var("USER").unwrap();

    loop {
        print!("$ {user_name}");

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
    }
    Ok(())
}
