use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
mod error;
mod types;
use log;
use types::request::{Command, CommandFactory};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);
    let mut request = String::new();
    if buf_reader.read_line(&mut request).is_err() {
        return;
    }
    log::info!("Request: {}", request);

    let cmd = CommandFactory::new(&vec!["PING".to_string()]);
    match cmd {
        Ok(c) => {
            let msg: String = (*c.execute()).to_string();
            println!("{}", msg);
            stream.write_all(&msg.as_bytes()).unwrap();
        }
        Err(e) => log::error!("Error parsing request: {}", e),
    }
}
