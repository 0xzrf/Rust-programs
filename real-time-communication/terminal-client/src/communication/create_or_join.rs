use super::structs::*;
use crate::{
    errors::{CreateErrors, JoinErrors, OnboardErrors},
    user_onboard::print_help,
};
use std::{
    collections::HashMap,
    env,
    io::{self, Write},
    sync::{Arc, RwLock},
};

pub struct Communication {
    pub rooms: SharedServer,
}

impl Communication {
    pub fn build() -> Self {
        let rooms = Arc::new(RwLock::new(ServerState {
            rooms: HashMap::new(),
        }));
        Communication { rooms }
    }

    /// This is the place that will handle continuousely asking user for the command they want to use
    /// It requres no arguments, but has the possibility of erroring out
    pub async fn user_response_onboarding(&self) -> Result<(), OnboardErrors> {
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
                "/create" => self.create_room(&user_name).await?,
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

    async fn create_room(&self, username: &str) -> Result<(), CreateErrors> {
        Ok(())
    }

    fn join_room(username: &str) -> Result<(), JoinErrors> {
        todo!()
    }
}
