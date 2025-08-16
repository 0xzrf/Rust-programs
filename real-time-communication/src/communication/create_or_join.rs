use crate::{
    errors::{CreateErrors, JoinErrors, OnboardErrors},
    user_onboard::print_help,
};
use std::{
    env,
    io::{self, Write},
};

pub struct Communication;

impl Communication {
    pub fn build() -> Self {
        Communication
    }

    /// This is the place that will handle continuousely asking user for the command they want to use
    /// It requres no arguments, but has the possibility of erroring out
    pub fn user_response_onboarding(&self) -> Result<(), OnboardErrors> {
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

            // TODO: Create match arms for cases of error
            match cmd {
                "/create" => Self::create_room(&user_name)?,
                "/join" => Self::join_room(&user_name)?,
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

    fn create_room(username: &str) -> Result<(), CreateErrors> {
        todo!()
    }

    fn join_room(username: &str) -> Result<(), JoinErrors> {
        todo!()
    }
}
