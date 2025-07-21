use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Create a new TcpListener (note that 'bind' here means connecting to a port, or binding to a port)
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // Binding to a port can return an error, since some ports require admin priviliges, sometimes there could be more than one tcp connection at one port, etc.

    // Loop through each stream in the TCP connection (each stream is an open connection between client and server)
    for stream in listener.incoming() {
        // Can return an error when stream isn't an actual stream, but a connection attempt
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Initialize a buffer of 512 bytes, each initialized to 0
    let mut buffer = [0; 512];

    // Read the stream into the buffer
    stream.read(&mut buffer).unwrap();

    // Print out the request contents
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
