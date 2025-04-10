use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool{
    worker: Vec<Worker>, // Since the closure won't return a value, we will have the () as the return value
    sender: Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>
}

impl Drop for ThreadPool {

    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.worker {

            println!("Shutting down the worker iwith ID: {}", worker.id);

            if let Some(val) = worker.handle.take() {
                val.join().unwrap();
            }

        }

    }
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
        if limit == 0 {
            return Err(PoolCreationError::InvalidLimit);
        }
        
        let (sender, receiver) = mpsc::channel();

        let mut worker = Vec::with_capacity(limit);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..limit {
            worker.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool{ worker, sender: Some(sender) })
    }

    /// Takes a closure and then sends it to the Worker
    /// 
    /// this method sends the closure passed to the execute function to the Worker available
    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static, 
        {
            let job = Box::new(f);

            self.sender.as_ref().unwrap().send(job).unwrap();

        }
}


impl Worker {
    /// This method takes an id and a receiver
    /// 
    /// The receiver is safe to use since it is inside Mutex, which restricts usage from multiple places at once`
    /// 
    /// Hence avoiding multiple usage of receiver at once
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>> ) -> Worker {

        let handle = thread::spawn(move|| loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} receired a job, executing");

            job();
        });

        Worker {
            id,
            handle: Some(handle)
        }
    }
}

/// We could include more errors and handle the Result in a better manner
/// for now, that's all I got
#[derive(Debug)]
pub enum PoolCreationError {
    InvalidLimit
}

