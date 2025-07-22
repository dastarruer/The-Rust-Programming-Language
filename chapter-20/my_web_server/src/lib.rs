use std::thread;

/// An implementation of a thread pool, which is a 'pool' of threads that wait for code to be given to them and executed.
pub struct ThreadPool {
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
