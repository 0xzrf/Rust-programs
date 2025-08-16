use crate::errors::OnboardErrors;
use std::{
    env,
    io::{self, Write},
};

pub struct Communication;

impl Communication {
    pub fn user_response_onboarding() {
        let user_name = env::var("USER").unwrap();

        loop {
            print!("┌─[{user_name}]─]\n└─▶ ");
            io::stdout().flush().unwrap(); // Force flush

            // Wait for user input
            let stdin = io::stdin();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();

            match &input[..] {
                "/create" => {}
                "/join" => {}
                "/help" => {}
                _ => {}
            }
        }
    }
}
