extern crate my_web_server;
use my_web_server::ThreadPool;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a new TcpListener (note that 'bind' here means connecting to a port, or binding to a port)
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap(); // Binding to a port can return an error, since some ports require admin priviliges, sometimes there could be more than one tcp connection at one port, etc.

    // Create a thread pool; by limiting the number of threads available we minimize the risk of overwhelming our system with unlimited threads
    let pool = ThreadPool::new(4);

    // Loop through each stream in the TCP connection (each stream is an open connection between client and server)
    for stream in listener.incoming() {
        // Can return an error when stream isn't an actual stream, but a connection attempt
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Initialize a buffer of 512 bytes, each initialized to 0
    let mut buffer = [0; 512];

    // Read the stream into the buffer
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // If the request is a GET request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        // Simulate long operation
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // Open the html file
    let mut file = File::open(filename).unwrap();

    // Read the contents of the html into `contents`
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Append the html to the response
    let response = format!("{}{}", status_line, contents);

    // Send the response
    stream.write(response.as_bytes()).unwrap();

    // Prevent the program from continuing until all bytes are sent
    stream.flush().unwrap();
}
