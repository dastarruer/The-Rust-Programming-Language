use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a new Arc so this Mutex can be shared across threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Create a new reference to counter
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Unlock the mutex
            let mut num = counter.lock().unwrap();

            // Add one to the value inside the mutex
            *num += 1;
        });
        handles.push(handle);
    }

    // Block main thread until all threads are finished; without this undefined behavior can occur
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
