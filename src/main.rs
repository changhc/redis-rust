use env_logger;
use redis_rust::utils::*;
use std::net::TcpListener;

fn main() {
    env_logger::init();
    let mut data_store = load_data_store();
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        match handle_connection(&stream, &mut data_store) {
            Ok(_) => (),
            Err(e) => handle_error(stream, e),
        };
    }
}
