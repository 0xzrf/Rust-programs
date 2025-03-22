use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool{
    worker: Vec<Worker>, // Since the closure won't return a value, we will have the () as the return value
    sender: mpsc::Sender<Job>
}

struct Job;

pub struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>
}


impl ThreadPool {
    /// Creates a new ThreadPool
    ///
    /// The limit is the number of threads in the pool.
    ///
    /// #Panics 
    ///
    /// The new function will panic if the limit is zero.
    pub fn build(limit: usize) -> Result<ThreadPool, PoolCreationError> {
        if limit > 0 {
            return Err(PoolCreationError::InvalidLimit);
        }
        
        let (sender, receiver) = mpsc::channel();

        let mut worker = Vec::with_capacity(limit);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..limit {
            worker.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool{ worker, sender })
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static, 
        {
            
        }
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>> ) -> Worker {

        let handle = thread::spawn(|| {
            receiver;
        });

        Worker {
            id,
            handle
        }
    }
}


#[derive(Debug)]
pub enum PoolCreationError {
    InvalidLimit
}

