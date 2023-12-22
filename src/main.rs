use log;
use redis_rust::command::{Command, CommandFactory};
use redis_rust::parse_request;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let tokens = parse_request(&stream);
    let cmd = CommandFactory::new(&vec!["PING".to_string()]);
    match cmd {
        Ok(c) => {
            let msg: String = (*c.execute()).serialise();
            println!("{}", msg);
            stream.write_all(&msg.as_bytes()).unwrap();
        }
        Err(e) => log::error!("Error parsing request: {}", e),
    }
}
