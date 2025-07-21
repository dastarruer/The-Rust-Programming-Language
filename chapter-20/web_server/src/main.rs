use std::fs::File;
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

    let get = b"GET / HTTP/1.1\r\n";

    // If the request is a GET request
    if buffer.starts_with(get) {
        // Open the html file
        let mut file = File::open("hello.html").unwrap();

        // Read the contents of the html into `contents`
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Append the html to the response
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        // Send the response
        stream.write(response.as_bytes()).unwrap();

        // Prevent the program from continuing until all bytes are sent
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open("404.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
