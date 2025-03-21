pub struct ThreadPool;


impl ThreadPool {
    /// Creates a new ThreadPool
    ///
    /// The limit is the number of threads in the pool.
    ///
    /// #Panics 
    ///
    /// The new function will panic if the limit is zero.
    pub fn new(limit: usize) -> ThreadPool {
        assert!(size > 0);


        ThreadPool
    }

    pub fn execute<F>(&self, f: F) 
    where
        F: FnOnce() + Send + 'static, 
        {

        }
}
