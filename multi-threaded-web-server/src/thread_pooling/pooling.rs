use crate::handle_connection;
use std::{net::TcpStream, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Creates the thread pools for a given number of threads
    ///
    /// size is the number of pools to create
    ///
    /// the `new` function will panic if the size == 0
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            // This should be fine because we're not expected to give a huge number for thread size
            let worker = Worker::new(i);
            workers.push(worker);
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
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
    fn new(id: usize) -> Worker {
        let handle = thread::spawn(|| {});

        Worker { id, thread: handle }
    }
}
