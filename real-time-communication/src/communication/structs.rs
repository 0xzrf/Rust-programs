use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

pub struct ServerState {
    pub rooms: Option<HashMap<String, Arc<Mutex<Room>>>>, // room name -> room
}
pub type SharedServer = Arc<RwLock<Option<ServerState>>>;

pub struct Room {
    pub user: Vec<String>,
    pub creator: String,
    pub msgs: Vec<ServerMsgs>,
}

pub struct ServerMsgs {
    pub user: String,
    pub msg: String,
}
