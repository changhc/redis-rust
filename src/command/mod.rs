mod base;
mod ping;
use super::error::RequestError;
pub use base::Command;
use ping::PingCommand;
mod set;
use set::SetCommand;
mod types;
use std::str::FromStr;
use types::CommandType;

#[derive(Debug)]
pub struct CommandFactory;

impl CommandFactory {
    pub fn new(tokens: &Vec<String>) -> Result<Box<dyn Command>, Box<dyn std::error::Error>> {
        let command = tokens[0].clone();
        let body = tokens[1..tokens.len()].into();
        match CommandType::from_str(&command) {
            Ok(c) => match c {
                CommandType::PING => Ok(Box::new(PingCommand::new(body))),
                CommandType::SET => match SetCommand::new(body) {
                    Ok(v) => Ok(Box::new(v)),
                    Err(e) => Err(Box::new(RequestError::InvalidCommand(
                        command,
                        e.to_string(),
                    ))),
                },
            },
            Err(_) => Err(Box::new(RequestError::UnsupportedCommand(command))),
        }
    }
}
