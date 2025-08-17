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
use tokio::net::{TcpListener, TcpStream};

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

        let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

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
                "/create" => self.create_room(&user_name, &listener).await?,
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

    async fn create_room(
        &self,
        username: &str,
        listener: &TcpListener,
    ) -> Result<(), CreateErrors> {
        loop {
            let (stream, _) = listener.accept().await.expect("Listener.accept fucked up");
            let state = self.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_client(stream, state).await {
                    eprintln!("client error: {e:?}");
                }
            });
        }
    }

    async fn handle_client(stream: TcpStream, state: SharedServer) -> Result<(), Error> {
        todo!()
    }

    fn join_room(username: &str) -> Result<(), JoinErrors> {
        todo!()
    }
}
