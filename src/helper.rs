use super::error::RequestError;
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

pub fn parse_request(stream: &TcpStream) -> Result<Vec<String>, RequestError> {
    let array_regex = Regex::new(r"^\*(\d+)\r\n$").unwrap();
    let bulk_string_regex = Regex::new(r"^\$(\d+)\r\n$").unwrap();
    let mut buf_reader = BufReader::new(stream);
    let mut length_line = String::new();
    if buf_reader.read_line(&mut length_line).is_err() {
        return Err(RequestError::Unknown);
    }

    let token_count = match array_regex.captures(&length_line) {
        Some(cap) => match cap[1].parse::<u32>() {
            Ok(v) => v,
            Err(e) => return Err(RequestError::Unknown),
        },
        None => return Err(RequestError::Unknown),
    };

    let mut tokens: Vec<String> = vec![];
    for _ in 0..token_count {
        let mut length_line = String::new();
        let mut req_body: String = String::new();
        if buf_reader.read_line(&mut length_line).is_err() {
            break;
        }
        let token_length = match bulk_string_regex.captures(&length_line) {
            Some(cap) => match cap[1].parse::<usize>() {
                Ok(v) => v,
                Err(e) => return Err(RequestError::Unknown),
            },
            None => return Err(RequestError::Unknown),
        };

        if buf_reader.read_line(&mut req_body).is_err() {
            break;
        }
        match req_body.get(0..token_length) {
            Some(s) => tokens.push(s.to_string()),
            None => return Err(RequestError::Unknown),
        };
    }
    Ok(tokens)
}
