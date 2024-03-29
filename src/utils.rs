use crate::execution_result::ExecutionResult;

use super::command::CommandFactory;
use super::error::RequestError;
use super::execution_result::ErrorResult;
use crate::data_store::DataStore;
use log;
use regex::Regex;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::net::tcp::ReadHalf;
use tokio::net::tcp::WriteHalf;

pub fn handle_error(stream: &WriteHalf<'_>, error_message: String) {
    log::error!("Error: {}", &error_message);
    let err = ErrorResult {
        message: error_message,
    };
    // If the error message cannot be written to the stream somehow, just let it go.
    let _ = stream.try_write(err.serialise().as_bytes());
}

pub async fn handle_connection(
    mut rx: ReadHalf<'_>,
    tx: &WriteHalf<'_>,
    data_store: Arc<Mutex<DataStore>>,
) -> Result<(), String> {
    let array_regex: Regex = Regex::new(r"^\*(\d+)\r\n$").unwrap();
    let bulk_string_regex: Regex = Regex::new(r"^\$(\d+)\r\n$").unwrap();
    loop {
        match parse_request(&mut rx, &array_regex, &bulk_string_regex).await {
            Ok(Some(tokens)) => {
                log::info!("tokens: {:?}", tokens);
                let cmd = CommandFactory::new(&tokens);
                match cmd {
                    Ok(c) => match c.execute(&mut data_store.lock().unwrap()) {
                        Ok(res) => {
                            let msg: String = (*res).serialise();
                            log::info!("response: {}", msg);
                            tx.try_write(msg.as_bytes()).unwrap();
                        }
                        Err(e) => return Err(e.to_string()),
                    },
                    Err(e) => return Err(e.to_string()),
                }
            }
            Ok(None) => break,
            Err(e) => return Err(e.to_string()),
        };
    }
    log::info!("Connection dropped");
    Ok(())
}

pub async fn parse_request(
    stream: &mut ReadHalf<'_>,
    array_regex: &Regex,
    bulk_string_regex: &Regex,
) -> Result<Option<Vec<String>>, RequestError> {
    let mut buf_reader = BufReader::new(stream);
    let mut length_line = String::new();
    match buf_reader.read_line(&mut length_line).await {
        // No bytes read from the stream; EOF is received: this connection is closed, which means
        // no more command from this client so we can gracefully return.
        Ok(0) => return Ok(None),
        Ok(_) => (),
        Err(e) => {
            return Err(RequestError::ParseRequestFailed(
                "read token count".to_string(),
                e.to_string(),
            ))
        }
    };

    let token_count = match array_regex.captures(&length_line) {
        Some(cap) => match cap[1].parse::<u32>() {
            Ok(v) => v,
            Err(e) => {
                return Err(RequestError::ParseRequestFailed(
                    "convert token count to u32".to_string(),
                    e.to_string(),
                ))
            }
        },
        None => {
            return Err(RequestError::ParseRequestFailed(
                "parse token count".to_string(),
                "none".to_string(),
            ));
        }
    };

    let mut tokens: Vec<String> = vec![];
    for i in 0..token_count {
        let mut length_line = String::new();
        let mut req_body: String = String::new();
        match buf_reader.read_line(&mut length_line).await {
            // EOF here would be unexpected and should be treated as an invalid string.
            Ok(0) => return Ok(None),
            Ok(_) => (),
            Err(e) => {
                return Err(RequestError::ParseRequestFailed(
                    format!("read size of token {}", i).to_string(),
                    e.to_string(),
                ))
            }
        };
        let token_length = match bulk_string_regex.captures(&length_line) {
            Some(cap) => match cap[1].parse::<usize>() {
                Ok(v) => v,
                Err(e) => {
                    return Err(RequestError::ParseRequestFailed(
                        format!("convert size of token {} to usize", i).to_string(),
                        e.to_string(),
                    ))
                }
            },
            None => {
                return Err(RequestError::ParseRequestFailed(
                    format!("parse size of token {}", i).to_string(),
                    "none".to_string(),
                ))
            }
        };

        match buf_reader.read_line(&mut req_body).await {
            // EOF here would be unexpected and should be treated as an invalid string.
            Ok(0) => return Ok(None),
            Ok(_) => (),
            Err(e) => {
                return Err(RequestError::ParseRequestFailed(
                    format!("read body of token {}", i).to_string(),
                    e.to_string(),
                ))
            }
        };
        match req_body.get(0..token_length) {
            Some(s) => tokens.push(s.to_string()),
            None => {
                return Err(RequestError::ParseRequestFailed(
                    format!("extract body of token {}", i).to_string(),
                    "none".to_string(),
                ))
            }
        };
    }
    Ok(Some(tokens))
}
