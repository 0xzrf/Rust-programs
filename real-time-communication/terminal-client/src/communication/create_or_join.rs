use crate::{
    errors::{CreateErrors, JoinErrors, OnboardErrors},
    user_onboard::print_help,
};
use futures::future::join;
use std::{
    io::{self, Read, Write},
    time::Duration,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::{net::TcpStream, time::sleep};
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

            let stream = self.connect_server().await.unwrap();

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
        println!("Joining room");
        let join_msg = format!(r#"{{"type":"JoinRoom","room":"{room}"}}{}"#, "\n");
        let join_bfr = join_msg.as_bytes();

        stream
            .write_all(join_bfr)
            .await
            .expect("Couldn't send buffer");
        stream.flush().await.unwrap();

        let (reader, mut writer) = stream.into_split();

        let user_name = self.user_name.clone();
        let room_write = String::from(room);

        let read_task = tokio::task::spawn(async move {
            let mut buf_reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                line.clear();
                match buf_reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("Connection closed by server");
                        break;
                    }
                    Ok(_) => {
                        println!("Received: {}", line.trim());
                    }
                    Err(e) => {
                        eprintln!("Read error: {e}");
                        break;
                    }
                }
            }
        });
        let write_task = tokio::task::spawn(async move {
            loop {
                print!("$ ");
                io::stdout().flush().unwrap(); // Force flush
                let stdin = tokio::io::stdin();
                let mut reader = BufReader::new(stdin).lines();

                while let Ok(Some(line)) = reader.next_line().await {
                    let msg = format!(
                        r#"{{"type":"Message","room":"{room_write}","from":"{user_name}","text":"{line}"}}{}"#,
                        "\n"
                    );

                    if let Err(e) = writer.write_all(msg.as_bytes()).await {
                        eprintln!("Write error: {e}");
                        break;
                    }
                    if let Err(e) = writer.flush().await {
                        eprintln!("Flush error: {e}");
                        break;
                    }

                    println!("Sent: {line}");
                }
            }
        });

        join(read_task, write_task).await;

        Ok(())
    }

    async fn connect_server(&self) -> Result<TcpStream, OnboardErrors> {
        // Connect to the first nc listener (terminal 1)
        if let Ok(stream) = TcpStream::connect("127.0.0.1:8080").await {
            return Ok(stream);
        }
        println!("Couldn't return");
        Err(OnboardErrors::ServerError("Couldn't return"))
    }
}
