#[cfg(test)]
mod communication_tests {
    use terminal_client::{Communication, Messages};
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    #[tokio::test]
    async fn test_join_after_connect_successfully() {
        let mut stream = Communication::connect_server().await.unwrap();

        let (read, mut writer) = stream.split();

        let room = format!("Room");

        let create_json = serde_json::json!({
            "type": "CreateRoom",
            "room": "Unknown",
        })
        .to_string()
            + "\n";
        let create_bfr = create_json.as_bytes();

        writer.write_all(create_bfr).await.unwrap();

        let mut buf_reader = BufReader::new(read);
        let mut line = String::new();
        match buf_reader.read_line(&mut line).await {
            Ok(_) => {
                let msg: Messages = match serde_json::from_str(line.trim()) {
                    Ok(c) => c,
                    Err(_) => {
                        panic!("Couldn't parse message");
                    }
                };

                match msg {
                    Messages::Created { room } => assert_eq!(room, "Unknow".to_string()),
                    _ => {
                        panic!();
                    }
                }
            }
            Err(e) => {
                eprintln!("Read error: {e}");
            }
        }
    }
}
