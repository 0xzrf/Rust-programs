use tokio::{sync::mpsc::UnboundedSender, task, task::JoinHandle};

pub struct Worker {
    pub task: JoinHandle<()>,
    pub id: usize,
}

pub struct STAsyncIO {
    workers: Vec<Worker>,
    sender: UnboundedSender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl STAsyncIO {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        todo!()
    }
}

impl Worker {
    pub async fn new(id: usize) -> Self {
        let task = task::spawn(async {});

        Worker { id, task }
    }
}
