use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ServerState {
    pub rooms: HashMap<String, Arc<Mutex<Room>>>, // room name -> room
}

pub struct Room {
    pub user: Vec<String>,
    pub creator: String,
    pub msgs: Vec<ServerMsgs>,
}

pub struct ServerMsgs {
    pub user: String,
    pub msg: String,
}
