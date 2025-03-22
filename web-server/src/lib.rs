use std::thread;

pub struct ThreadPool{
    threads: Vec<Worker>, // Since the closure won't return a value, we will have                                           // the () as the return value
}

pub struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize) -> Worker {

        let handle = thread::spawn(|| {
            ()
        });

        Worker {
            id,
            handle
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
        if limit > 0 {
            return Err(PoolCreationError::InvalidLimit);
        }
        
        let mut threads = Vec::with_capacity(limit);

        for i in 0..limit {
            threads.push(Worker::new(i));
        }

        Ok(ThreadPool{ threads })
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static, 
        {
            
        }
}



#[derive(Debug)]
pub enum PoolCreationError {
    InvalidLimit
}

