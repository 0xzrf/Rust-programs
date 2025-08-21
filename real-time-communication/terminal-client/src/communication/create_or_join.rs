use crate::{
    communication::structs::Messages,
    errors::{CreateErrors, JoinErrors, OnboardErrors},
    helper::{get_input, print_center, print_right},
    user_onboard::print_help,
};

use serde_json::json;
use std::{
    io::{self as std_io, Write},
    sync::Arc,
    time::Duration,
};
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
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
        loop {
            print!("┌─[{}]─]\n└─▶ ", self.user_name);
            std_io::stdout().flush().unwrap(); // Force flush

            let input = get_input();
            let (cmd, arg) = input.split_once(" ").unwrap_or((&input, ""));

            let stream = match self
                .connect_server()
                .await
                .map_err(|_| OnboardErrors::ServerError("Couldn't connect to the server"))
            {
                Ok(tcp_stream) => tcp_stream,
                Err(err) => return Err(err),
            };

            match cmd.trim() {
                "/create" => self.create_room(arg, stream).await?,
                "/join" => self.join_room(arg, stream).await?,
                "/help" => {
                    print_help();
                }
                "/set_user" => {
                    self.user_name = arg.to_string();
                }
                "/quit" => {
                    println!("Quiting app...");
                    break;
                }
                _ => println!("Invalid command"),
            }
        }
        Ok(())
    }

    /// This function is used to join the room in the server
    /// It will simply send some TCP requests to it and then start messaging it
    async fn create_room(&mut self, room: &str, mut stream: TcpStream) -> Result<(), CreateErrors> {
        let create_json = json!({
            "type": "CreateRoom",
            "room": room,
        })
        .to_string()
            + "\n";
        let create_bfr = create_json.as_bytes();

        stream
            .write_all(create_bfr)
            .await
            .expect("Couldn't send buffer");
        stream.flush().await.unwrap();

        self.join_room(room, stream)
            .await
            .map_err(|_| CreateErrors::RoomNotCreated("Room not created"))
    }

    async fn join_room(&mut self, room: &str, mut stream: TcpStream) -> Result<(), JoinErrors> {
        print_center(&format!("Joining room: {room}"));
        self.joined_room = Some(String::from(room));
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

        let (reader, writer) = stream.into_split();

        let (user_name, room_write) = (
            Arc::new(RwLock::new(self.user_name.clone())),
            Arc::new(RwLock::new(String::from(room))),
        );

        let (username_clone_read, username_clone_write, room_write_clone, room_read_clone) = (
            Arc::clone(&user_name),
            Arc::clone(&user_name),
            Arc::clone(&room_write),
            Arc::clone(&room_write),
        );
        let mut write_task_handle = tokio::spawn(Self::write_task(
            writer,
            room_write_clone,
            username_clone_write,
        ));

        // We're using tokio::select! because it stops the async function the moment one of them stops.
        tokio::select! {
            biased;
            _ = Self::read_task(reader, room_read_clone, username_clone_read) => {
                write_task_handle.abort();
            },
            _ = &mut write_task_handle => {

            }
        }

        Ok(())
    }

    async fn connect_server(&self) -> Result<TcpStream, OnboardErrors> {
        // Connect to the first nc listener (terminal 1)
        if let Ok(stream) = TcpStream::connect("https://mean-rats-glow.loca.lt").await {
            return Ok(stream);
        }
        Err(OnboardErrors::ServerError("Couldn't return"))
    }

    async fn read_task(
        reader: OwnedReadHalf,
        room_read_clone: Arc<RwLock<String>>,
        username_clone_read: Arc<RwLock<String>>,
    ) {
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

                    // println!("Msg received from server: {msg:?}");

                    match msg {
                        Messages::Message { from, text } => {
                            let room_write = room_read_clone.read().await;

                            let user_name = username_clone_read.read().await;
                            let user_output = format!("[{from}]");
                            print_right(&user_output);
                            print_right(&text);
                            print!("┌─[{user_name}]─{room_write}");
                        }
                        Messages::Error { msg } => {
                            print_center(&msg);
                            break;
                        }
                        Messages::Joined { room } => {
                            print_center(&format!("Joined room: {room}"));
                        }
                        Messages::Created { room } => {
                            print_center(&room);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Read error: {e}");
                    break;
                }
            }
        }
    }

    async fn write_task(
        mut writer: OwnedWriteHalf,
        room_write_clone: Arc<RwLock<String>>,
        username_clone_write: Arc<RwLock<String>>,
    ) {
        // adding sleep here because when the read and write task start, the joined msg received create a bit of abrupt user interface
        tokio::time::sleep(Duration::from_millis(100)).await;
        loop {
            let user_name = username_clone_write.read().await;
            let room_write = room_write_clone.read().await;
            // io::stdout().flush().await.unwrap();

            let mut line = String::new();
            print!("┌─[{user_name}]─{room_write}");
            let bytes_read = io::BufReader::new(io::stdin())
                .read_line(&mut line)
                .await
                .unwrap();

            if bytes_read == 0 {
                break;
            }

            if line.trim().eq_ignore_ascii_case("/leave") {
                println!("Leaving room");
                break;
            }

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
    }
}
