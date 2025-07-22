use core::num;
use std::thread;

/// An implementation of a thread pool, which is a 'pool' of threads that wait
/// for code to be given to them and executed.
pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new instance of `ThreadPool`.
    ///
    /// # Parameters
    /// - `num_threads` - The number of threads that will be held by the
    /// `ThreadPool`.
    ///
    /// # Panics
    /// Panics if `num_threads` < 1.
    pub fn new(num_threads: usize) -> ThreadPool {
        // Panic if the number of threads is less than 1
        assert!(num_threads > 0);

        // Create a pre-allocated vector to hold each worker
        let mut workers = Vec::with_capacity(num_threads);

        // Create new workers and push them onto `workers`
        for id in 0..num_threads {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }

    /// Dispatch a job to be executed by `ThreadPool`.
    pub fn execute<F>(&self, _job: F)
    where
        F: FnOnce(), // `F` is a closure that will consume all values
    {
    }
}

/// An implementation of a 'worker' intended to be used in `ThreadPool`.
///
/// Each worker has its own `id` to uniquely identify it, and owns a `thread`,
/// which is used to execute jobs dispatched by `ThreadPool`
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new instance of `Worker`
    ///
    /// # Parameters
    /// - `id` - A unique id to identify each `Worker`.
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
