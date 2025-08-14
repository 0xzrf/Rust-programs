use crate::handle_connection;
use std::sync::{Arc, Mutex};
use std::{net::TcpStream, sync::mpsc, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers.drain(..) {
            println!("Stopping worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

impl ThreadPool {
    /// Creates the thread pools for a given number of threads
    ///
    /// size is the number of pools to create
    ///
    /// the `new` function will panic if the size == 0
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        let recv = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            // This should be fine because we're not expected to give a huge number for thread size
            let worker = Worker::new(i, Arc::clone(&recv));
            workers.push(worker);
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .expect("Couldn't send the job");
    }

    pub fn run_thread_pooling(&self, stream: TcpStream) {
        self.execute(|| {
            handle_connection(stream);
        });
    }
}

struct Worker {
    pub id: usize,
    pub thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || {
            loop {
                match receiver.lock().expect("Failed to unlock").recv() {
                    Ok(job) => {
                        println!("Worker {id} got a job. executing....");

                        job();
                    }
                    Err(_) => {
                        println!("Worker Id {id} is disconnected, shutting down");
                        break;
                    }
                }
            }
        });

        Worker { id, thread: handle }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
