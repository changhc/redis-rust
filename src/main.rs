
use redis_rust::data_store;
use redis_rust::utils;

use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();
    let data_store = Arc::new(Mutex::new(data_store::DataStore::new()));
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    while let Ok((mut stream, _address)) = listener.accept().await {
        // Clone the arc here so that `data_store` does not get moved during the first spawn.
        let ds_clone = data_store.clone();
        tokio::spawn(async move {
            let (rx, tx) = stream.split();
            match utils::handle_connection(rx, &tx, ds_clone).await {
                Ok(_) => (),
                Err(e) => utils::handle_error(&tx, e),
            };
        });
    }
}
