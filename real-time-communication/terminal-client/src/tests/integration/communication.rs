#[cfg(test)]
mod communication_tests {
    use std::sync::Arc;
    use tokio::sync::RwLock;

    use rand::Rng;
    use terminal_client::Communication;

    #[tokio::test]
    async fn test_join_after_connect_successfully() {
        let mut stream = Communication::connect_server().await.unwrap();

        let (read, mut writer) = stream.into_split();

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
    }
}
