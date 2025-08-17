use crate::{
    errors::{CreateErrors, JoinErrors, OnboardErrors},
    user_onboard::print_help,
};
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

            match cmd {
                "/create" => self.create_room().await?,
                "/join" => self.join_room()?,
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
    async fn create_room(&self) -> Result<(), CreateErrors> {
        self.connect_server()
            .await
            .expect("Couldn't connect to server");
        Ok(())
    }

    fn join_room(&self) -> Result<(), JoinErrors> {
        todo!()
    }

    async fn connect_server(&self) -> Result<(), OnboardErrors> {
        // Connect to the first nc listener (terminal 1)
        if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
            let mut str_clone = stream.try_clone().unwrap();
            let handle = tokio::task::spawn(async move {
                loop {
                    let mut buffer = [0; 512];

                    while let Ok(n) = stream.read(&mut buffer) {
                        if n == 0 {
                            println!("Closing connection");
                            break; // connection closed
                        }
                        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                        stream.flush().unwrap();
                    }
                }
            });

            println!("Joining and sending");
            let join_bfr = b"{\"type\":\"JoinRoom\",\"room\":\"lobby\"}\n";
            let msg_bfr = b"{\"type\":\"Message\",\"room\":\"lobby\",\"from\":\"server\",\"text\":\"hello terminal 1\"}\n";

            str_clone.write_all(join_bfr).expect("Couldn't send buffer");
            str_clone.flush().unwrap();

            for _ in 0..100 {
                println!("Sending buffer");
                str_clone.write_all(msg_bfr).expect("Couldn't send buffer");
                str_clone.flush().unwrap();
                sleep(Duration::from_secs(5)).await;
            }

            handle.await.unwrap();
        } else {
            eprintln!("Could not connect to listener 1");
        }

        Ok(())
    }
}
