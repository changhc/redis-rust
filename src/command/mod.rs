mod base;
mod ping;
use super::error::RequestError;
pub use base::Command;
use ping::PingCommand;
mod types;
use std::str::FromStr;
use types::CommandType;

#[derive(Debug)]
pub struct CommandFactory;

impl CommandFactory {
    pub fn new(tokens: &Vec<String>) -> Result<impl Command, RequestError> {
        match CommandType::from_str(&tokens[0]) {
            Ok(c) => match c {
                CommandType::PING => Ok(PingCommand::new(tokens[1..tokens.len()].into())),
            },
            Err(_) => Err(RequestError::UnsupportedCommand(tokens[0].clone())),
        }
    }
}
