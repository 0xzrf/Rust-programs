use std::thread;

pub struct ThreadPool{
    thread: Vec<threads::JoinHandle<()>>, // Since the closure won't return a value, we will have
                                          // the () as the return value
};

#[derive(Debug)]
pub enum PoolCreationError {
    InvalidLimit
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
        


        Ok(ThreadPool)
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static, 
        {
            
        }
}
