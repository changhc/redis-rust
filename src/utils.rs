use crate::execution_result::ExecutionResult;

use super::command::CommandFactory;
use super::error::RequestError;
use super::execution_result::ErrorResult;
use crate::data_store::DataStore;
use log;
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

pub fn handle_error(mut stream: TcpStream, error_message: String) {
    log::error!("Error: {}", &error_message);
    let err = ErrorResult {
        message: error_message,
    };
    stream.write_all(err.serialise().as_bytes()).unwrap();
}

pub fn handle_connection(mut stream: &TcpStream, data_store: &mut DataStore) -> Result<(), String> {
    return match parse_request(stream) {
        Ok(tokens) => {
            log::info!("tokens: {:?}", tokens);
            let cmd = CommandFactory::new(&tokens);
            match cmd {
                Ok(c) => match c.execute(data_store) {
                    Ok(res) => {
                        let msg: String = (*res).serialise();
                        log::info!("response: {}", msg);
                        stream.write_all(&msg.as_bytes()).unwrap();
                        Ok(())
                    }
                    Err(e) => Err(e.to_string()),
                },
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    };
}

pub fn parse_request(stream: &TcpStream) -> Result<Vec<String>, RequestError> {
    let array_regex = Regex::new(r"^\*(\d+)\r\n$").unwrap();
    let bulk_string_regex = Regex::new(r"^\$(\d+)\r\n$").unwrap();
    let mut buf_reader = BufReader::new(stream);
    let mut length_line = String::new();
    match buf_reader.read_line(&mut length_line) {
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
            ))
        }
    };

    let mut tokens: Vec<String> = vec![];
    for i in 0..token_count {
        let mut length_line = String::new();
        let mut req_body: String = String::new();
        match buf_reader.read_line(&mut length_line) {
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

        match buf_reader.read_line(&mut req_body) {
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
    Ok(tokens)
}
