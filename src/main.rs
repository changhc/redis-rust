use log;
use redis_rust::parse_request;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    match parse_request(&stream) {
        Ok(tokens) => {
            println!("{:?}", tokens);
        }
        Err(e) => log::error!("Error parsing request: {}", e),
    }
}
