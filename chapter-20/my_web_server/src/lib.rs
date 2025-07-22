use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread,
};

/// Alias for a job's type
// We use a Box so that variables of this type can be sent across threads
// safely, as its size is known at compile time
type Job = Box<dyn FnOnce() + Send + 'static>;

/// An implementation of a thread pool, which is a 'pool' of threads that wait
/// for code to be given to them and executed.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
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

        // Create a channel that will send jobs to each worker
        let (sender, receiver) = mpsc::channel::<Job>();

        // Extract the receiver into a Arc<Mutex<T>> so that each thread can both read and write to it
        let receiver = Arc::new(Mutex::new(receiver));

        // Create a pre-allocated vector to hold each worker
        let mut workers = Vec::with_capacity(num_threads);

        // Create new workers and push them onto `workers`
        for id in 0..num_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
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
    /// - `receiver` - The receiving end of the channel that is used to send
    /// jobs to the worker
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_thread_pool_panics() {
        // Should panic because ThreadPool cannot take 0 threads
        let thread_pool = ThreadPool::new(0);
    }
}
