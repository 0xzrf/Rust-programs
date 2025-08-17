use crate::{
    errors::{CreateErrors, JoinErrors, OnboardErrors},
    user_onboard::print_help,
};
use futures::future::join;
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    time::Duration,
};

use tokio::time::sleep;
pub struct Communication {
    pub user_name: String,
    pub joined_room: Option<String>,
}

impl Communication {
    pub fn build(user_name: String) -> Self {
        Communication {
            user_name,
            joined_room: None,
        }
    }

    /// This is the place that will handle continuousely asking user for the command they want to use
    /// It requres no arguments, but has the possibility of erroring out
    pub async fn user_response_onboarding(&mut self) -> Result<(), OnboardErrors> {
        loop {
            print!("┌─[{}]─]\n└─▶ ", self.user_name);
            io::stdout().flush().unwrap(); // Force flush

            // Wait for user input
            let stdin = io::stdin();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let input = input.trim();
            let (cmd, arg) = input.split_once(" ").unwrap_or((input, ""));

            let stream = self.connect_server().unwrap();

            match cmd {
                "/create" => self.create_room(stream).await?,
                "/join" => self.join_room(arg, stream).await?,
                "/help" => {
                    print_help();
                }
                "/set_user" => {
                    self.user_name = arg.to_string();
                }
                _ => println!("Invalid command"),
            }
        }
    }

    /// This function is used to join the room in the server
    /// It will simply send some TCP requests to it and then start messaging it
    async fn create_room(&self, stream: TcpStream) -> Result<(), CreateErrors> {
        // Wait for user input
        println!("Input the room name:");
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        self.join_room(input, stream)
            .await
            .map_err(|_| CreateErrors::RoomNotCreated("Room not created"))
    }

    async fn join_room(&self, room: &str, mut stream: TcpStream) -> Result<(), JoinErrors> {
        let join_msg = format!(r#"{{"type":"JoinRoom","room":"{room}"}}\n"#);
        let join_bfr = join_msg.as_bytes();

        stream.write_all(join_bfr).expect("Couldn't send buffer");
        stream.flush().unwrap();

        let mut str_clone = stream.try_clone().unwrap();

        let read_task = tokio::task::spawn(async move {
            loop {
                let mut buffer = [0; 512];

                while let Ok(n) = str_clone.read(&mut buffer) {
                    if n == 0 {
                        println!("Closing connection");
                        break; // connection closed
                    }
                    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                    str_clone.flush().unwrap();
                }
            }
        });
        let user_name = self.user_name.clone();
        let room_write = String::from(room);

        let write_task = tokio::task::spawn(async move {
            loop {
                print!("$[{user_name}] ─▶");
                io::stdout().flush().unwrap(); // Force flush
                let stdin = io::stdin();
                let mut raw_msg_to_send = String::new();
                stdin.read_line(&mut raw_msg_to_send).unwrap();
                let msg_to_send = raw_msg_to_send.trim();
                let msg = format!(
                    r#"{{"type":"Message","room":"{room_write}","from":"server","text":"{msg_to_send}"}}\n"#,
                );

                let msg_bfr = msg.as_bytes();

                stream.write_all(msg_bfr).expect("Couldn't send buffer");
                println!("Sending message");
                stream.flush().unwrap();
            }
        });

        join(read_task, write_task).await;

        Ok(())
    }

    fn connect_server(&self) -> Result<TcpStream, OnboardErrors> {
        // Connect to the first nc listener (terminal 1)
        if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
            return Ok(stream);
        }
        println!("Couldn't return");
        Err(OnboardErrors::ServerError("Couldn't return"))
    }
}
