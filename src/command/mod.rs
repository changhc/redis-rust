mod base;
mod get;
mod ping;
use super::error::RequestError;
pub use base::Command;
use get::GetCommand;
use ping::PingCommand;
mod set;
use set::SetCommand;
mod incr;
use incr::IncrCommand;
mod types;
use std::str::FromStr;
use types::CommandType;

#[derive(Debug)]
pub struct CommandFactory;

impl CommandFactory {
    pub fn new(tokens: &Vec<String>) -> Result<Box<dyn Command>, RequestError> {
        let command = tokens[0].clone();
        let body = tokens[1..tokens.len()].into();
        match CommandType::from_str(&command) {
            Ok(c) => {
                let e = match c {
                    CommandType::PING => match PingCommand::new(body) {
                        Ok(v) => return Ok(v),
                        Err(e) => e,
                    },
                    CommandType::SET => match SetCommand::new(body) {
                        Ok(v) => return Ok(v),
                        Err(e) => e,
                    },
                    CommandType::GET => match GetCommand::new(body) {
                        Ok(v) => return Ok(v),
                        Err(e) => e,
                    },
                    CommandType::INCR => match IncrCommand::new(body) {
                        Ok(v) => return Ok(v),
                        Err(e) => e,
                    },
                };
                Err(RequestError::InvalidCommand(command, e.to_string()))
            }
            Err(_) => Err(RequestError::UnsupportedCommand(command)),
        }
    }
}
