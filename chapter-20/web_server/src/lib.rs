use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::thread::JoinHandle;

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // Unlock the mutex, then get the message in the receiver
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing...", id);

                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate...", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// # Parameters
    /// - `size` - The number of threads in the pool
    ///
    /// # Panics
    ///
    /// - The function will panic if `size` is smaller than 1.
    pub fn new(size: usize) -> ThreadPool {
        // Panic if the thread pool size is not bigger than 0
        assert!(size > 0);

        // Create a channel to send execution requests to each thread in the pool
        let (sender, receiver) = mpsc::channel();

        // Create an Arc for receiver so that multiple threads can access it
        let receiver = Arc::new(Mutex::new(receiver));

        // Allocate memory beforehand so that the vector does not have to spend resources resizing at runtime
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // Send the 'job' (just a closure) to all the workers
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending termination message to all workers...");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers...");

        for worker in &mut self.workers {
            println!("Shutting down worker {}...", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
