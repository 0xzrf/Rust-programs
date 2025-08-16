use crate::{errors::OnboardErrors, user_onboard::print_help};
use std::{
    env,
    io::{self, Write},
};

pub struct Communication;

impl Communication {
    pub fn user_response_onboarding() -> Result<(), OnboardErrors> {
        let mut user_name = env::var("USER").unwrap();

        loop {
            print!("┌─[{user_name}]─]\n└─▶ ");
            io::stdout().flush().unwrap(); // Force flush

            // Wait for user input
            let stdin = io::stdin();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let input = input.trim();
            let (cmd, arg) = input.split_once(" ").unwrap_or((input, ""));

            match cmd {
                "/create" => {}
                "/join" => {}
                "/help" => {
                    print_help();
                }
                "/set_user" => {
                    user_name = arg.to_string();
                }
                _ => println!("Invalid command"),
            }
        }
    }
}
