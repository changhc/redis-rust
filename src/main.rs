use env_logger;
use log;
use redis_rust::command::CommandFactory;
use redis_rust::parse_request;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn main() {
    env_logger::init();
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    match parse_request(&stream) {
        Ok(tokens) => {
            log::info!("tokens: {:?}", tokens);
            let cmd = CommandFactory::new(&tokens);
            match cmd {
                Ok(c) => {
                    let msg: String = (*c.execute()).serialise();
                    log::info!("response: {}", msg);
                    stream.write_all(&msg.as_bytes()).unwrap();
                }
                Err(e) => log::error!("Error parsing request: {}", e),
            }
        }
        Err(e) => log::error!("Error parsing request: {}", e),
    }
}
