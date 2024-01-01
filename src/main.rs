use env_logger;
use redis_rust::data_store;
use redis_rust::utils;

use tokio::io::*;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();
    let mut data_store = data_store::DataStore::new();
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    while let Ok((stream, _address)) = listener.accept().await {
        // this is similar to spawning a new thread.
        tokio::spawn(async {
            match utils::handle_connection(stream, &mut data_store).await {
                Ok(_) => (),
                Err(e) => utils::handle_error(stream, e),
            };
        });
    }
}
