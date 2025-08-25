use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

use rand::Rng;
use terminal_client::Communication;

async fn get_stream_and_room() -> (OwnedReadHalf, OwnedWriteHalf, String) {
    let stream = Communication::connect_server().await.unwrap();

    let (read, writer) = stream.into_split();

    let mut rng = rand::rng();
    let num: i32 = rng.random_range(1..=100);

    let room = format!("Room{num}");

    (read, writer, room)
}

#[cfg(test)]
mod communication_tests {
    use super::*;
    use std::sync::Arc;
    use terminal_client::Messages;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_create_after_connect_successfully() {
        let (mut read, writer, room) = get_stream_and_room().await;

        let writer_lock = Arc::new(RwLock::new(writer));
        let create_json = serde_json::json!({
            "type": "CreateRoom",
            "room": room,
        })
        .to_string()
            + "\n";

        Communication::send_msg(create_json, Arc::clone(&writer_lock))
            .await
            .unwrap();

        let msg = Communication::read_msg(&mut read).await.unwrap();

        if let Messages::Created { room: created_room } = msg {
            assert_eq!(created_room, room);
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn fails_when_room_doesnt_exist() {
        let (mut read, writer, room) = get_stream_and_room().await;

        let writer_lock = Arc::new(RwLock::new(writer));

        let create_json = serde_json::json!({
            "type": "JoinRoom",
            "room": room,
        })
        .to_string()
            + "\n";

        Communication::send_msg(create_json, Arc::clone(&writer_lock))
            .await
            .unwrap();

        let msg = Communication::read_msg(&mut read).await.unwrap();

        if let Messages::Error { msg } = msg {
            panic!();
        }
    }
}
