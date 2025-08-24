#[cfg(test)]
mod communication_tests {
    use std::sync::Arc;
    use tokio::sync::RwLock;

    use rand::Rng;
    use terminal_client::{Communication, Messages};

    #[tokio::test]
    async fn test_create_after_connect_successfully() {
        let stream = Communication::connect_server().await.unwrap();

        let (mut read, mut writer) = stream.into_split();

        let writer_lock = Arc::new(RwLock::new(writer));
        let mut rng = rand::thread_rng();
        let num: i32 = rng.gen_range(1..=100);

        let room = format!("Room{num}");

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
        // panic!()
    }
}
