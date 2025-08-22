use std::sync::Arc;

use crate::handle_connection;
use tokio::{
    net::TcpStream,
    sync::{
        Mutex,
        mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    },
    task::{self, JoinHandle},
};
pub struct Worker {
    pub task: JoinHandle<()>,
    pub id: usize,
}

pub struct STAsyncIO {
    workers: Vec<Worker>,
    sender: UnboundedSender<Job>,
}

impl STAsyncIO {
    pub async fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (tx, mut rx) = unbounded_channel();

        let rx_arc = Arc::new(Mutex::new(rx));

        for i in 0..size {
            let worker = Worker::new(i, Arc::clone(&rx_arc)).await;

            workers.push(worker);
        }

        Self {
            sender: tx,
            workers,
        }
    }

    pub async fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
    pub async fn run_st_async_io<F>(&self, stream: TcpStream)
    where
        F: FnOnce() + Send + 'static,
    {
        self.execute(|| {
            handle_connection(stream);
        });
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
impl Worker {
    pub async fn new(id: usize, recv: Arc<Mutex<UnboundedReceiver<Job>>>) -> Self {
        let task = task::spawn(async move {
            loop {
                let mut job_lock = recv.lock().await;

                if let Some(job) = job_lock.recv().await {
                    println!("Worker {id} gor the job, executing...");
                    job();
                }
            }
        });

        Worker { id, task }
    }
}
