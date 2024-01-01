mod base;
mod ping;
use crate::error::RequestError;
pub use base::Command;
use ping::PingCommand;
mod config_get;
use config_get::ConfigGetCommand;
mod types;
use std::str::FromStr;
use types::{CommandType, ListCommandType, StringCommandType};

mod list;
mod string;

#[derive(Debug)]
pub struct CommandFactory;

impl CommandFactory {
    pub fn new(tokens: &Vec<String>) -> Result<Box<dyn Command>, RequestError> {
        let command = tokens[0].clone();
        let body = tokens[1..tokens.len()].into();
        match CommandType::from_str(&command) {
            Ok(c) => match c {
                CommandType::PING => match PingCommand::new(body) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
                CommandType::CONFIG => match ConfigGetCommand::new(body) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
                CommandType::STRING(v) => handle_string_command(v, body),
                CommandType::LIST(v) => handle_list_command(v, body),
            },
            Err(_) => Err(RequestError::UnsupportedCommand(command)),
        }
    }
}

fn handle_string_command(
    v: StringCommandType,
    body: Vec<String>,
) -> Result<Box<dyn Command>, RequestError> {
    match v {
        StringCommandType::SET => match string::SetCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        StringCommandType::GET => match string::GetCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        StringCommandType::INCR => {
            match string::IncrCommand::new(body, string::NumOperator::INCR) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        StringCommandType::DECR => {
            match string::IncrCommand::new(body, string::NumOperator::DECR) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        StringCommandType::INCRBY => {
            match string::IncrbyCommand::new(body, string::NumOperator::INCR) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        StringCommandType::DECRBY => {
            match string::IncrbyCommand::new(body, string::NumOperator::DECR) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        StringCommandType::MGET => match string::MgetCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        StringCommandType::MSET => match string::MsetCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
    }
}

fn handle_list_command(
    v: ListCommandType,
    body: Vec<String>,
) -> Result<Box<dyn Command>, RequestError> {
    match v {
        ListCommandType::LPUSH => match list::LpushCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        ListCommandType::LPOP => match list::LpopCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        ListCommandType::LRANGE => match list::LrangeCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        ListCommandType::LLEN => match list::LlenCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
    }
}
