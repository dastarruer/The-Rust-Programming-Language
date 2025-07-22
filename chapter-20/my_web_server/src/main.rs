use std::net::TcpListener;

fn main() {
    println!("Starting server...");

    let port = "8080";
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    for incoming in listener.incoming() {
        let incoming = incoming.unwrap();

        println!("Request made: {:?}", incoming);
    }
}
