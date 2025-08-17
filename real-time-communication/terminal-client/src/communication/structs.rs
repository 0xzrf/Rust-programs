use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, mpsc};
use uuid::Uuid;

type ClientId = Uuid;
type Tx = mpsc::Sender<ServerMsgs>;
pub struct ServerState {
    pub rooms: HashMap<String, Arc<Mutex<Room>>>, // room name -> room
}
pub type SharedServer = Arc<RwLock<ServerState>>;

pub struct Room {
    pub user: Vec<String>,
    members: HashMap<ClientId, Tx>,
}

impl Room {
    fn join(&mut self, client_id: ClientId, tx: Tx) {}
    fn leave(&mut self, client_id: ClientId) {}
    fn broadcast(&mut self, msg: ClientId) {}
}

pub struct ServerMsgs {
    pub user: String,
    pub msg: String,
}
