use crate::{
    communication::structs::Messages,
    errors::{CreateErrors, JoinErrors, OnboardErrors},
    helper::{print_right, race},
    user_onboard::print_help,
};

use serde_json::json;
use std::{
    io::{self as std_io, Write},
    sync::Arc,
};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::RwLock,
};

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
        let mut done = false;
        while !done {
            print!("┌─[{}]─]\n└─▶ ", self.user_name);
            std_io::stdout().flush().unwrap(); // Force flush

            // Wait for user input
            let stdin = std_io::stdin();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let input = input.trim();
            let (cmd, arg) = input.split_once(" ").unwrap_or((input, ""));

            let stream = match self
                .connect_server()
                .await
                .map_err(|_| OnboardErrors::ServerError("Couldn't connect to the server"))
            {
                Ok(tcp_stream) => tcp_stream,
                Err(err) => return Err(err),
            };

            match cmd {
                "/create" => self.create_room(stream).await?,
                "/join" => self.join_room(arg, stream).await?,
                "/help" => {
                    print_help();
                }
                "/set_user" => {
                    self.user_name = arg.to_string();
                }
                "/quit" => {
                    println!("Quiting app...");
                    done = true;
                    break;
                }
                _ => println!("Invalid command"),
            }
        }
        Ok(())
    }

    /// This function is used to join the room in the server
    /// It will simply send some TCP requests to it and then start messaging it
    async fn create_room(&mut self, stream: TcpStream) -> Result<(), CreateErrors> {
        // Wait for user input
        println!("Input the room name:");
        let stdin = std_io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        self.join_room(input, stream)
            .await
            .map_err(|_| CreateErrors::RoomNotCreated("Room not created"))
    }

    async fn join_room(&mut self, room: &str, mut stream: TcpStream) -> Result<(), JoinErrors> {
        self.joined_room = Some(String::from(room));
        println!("Joined room: {room}");
        let join_msg = json!({
            "type": "JoinRoom",
            "room": room,
        })
        .to_string()
            + "\n";
        let join_bfr = join_msg.as_bytes();

        stream
            .write_all(join_bfr)
            .await
            .expect("Couldn't send buffer");
        stream.flush().await.unwrap();

        let (reader, mut writer) = stream.into_split();

        let (user_name, room_write) = (
            Arc::new(RwLock::new(self.user_name.clone())),
            Arc::new(RwLock::new(String::from(room))),
        );

        let (username_clone_read, username_clone_write, room_write_clone) = (
            Arc::clone(&user_name),
            Arc::clone(&user_name),
            Arc::clone(&room_write),
        );

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
                        let msg: Messages = match serde_json::from_str(line.trim()) {
                            Ok(c) => c,
                            Err(_) => {
                                continue;
                            }
                        };

                        match msg {
                            Messages::Message { from, text } => {
                                let user_name = username_clone_read.read().await;
                                let user_output = format!("[{from}]");
                                print_right(&user_output);
                                print_right(&text);
                                println!("┌─[{user_name}]─]");
                            }
                        }
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
                let user_name = username_clone_write.read().await;
                println!("┌─[{user_name}]─]");
                io::stdout().flush().await.unwrap();

                let mut line = String::new();
                let bytes_read = io::BufReader::new(io::stdin())
                    .read_line(&mut line)
                    .await
                    .unwrap();

                if bytes_read == 0 {
                    break; // EOF (Ctrl+D)
                }

                if line.trim().eq_ignore_ascii_case("/leave") {
                    println!("Leaving room");
                    break;
                }

                let room_write = room_write_clone.read().await;

                let msg = json!({
                    "type": "Message",
                    "room": *room_write,
                    "from": *user_name,
                    "text": line.trim()
                })
                .to_string()
                    + "\n";

                if let Err(e) = writer.write_all(msg.as_bytes()).await {
                    eprintln!("Write error: {e}");
                    break;
                }
                if let Err(e) = writer.flush().await {
                    eprintln!("Flush error: {e}");
                    break;
                }
            }
        });

        // We're racing the output here because once the write task ends, we need to stop the read task as well
        race(read_task, write_task).await;

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
