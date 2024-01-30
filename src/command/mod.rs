mod base;
mod ping;
use crate::error::RequestError;
pub use base::Command;
use ping::PingCommand;
mod types;
use std::str::FromStr;
use types::{CommandType, ListCommandType, StringCommandType};

use self::types::SetCommandType;

mod list;
mod set;
mod string;

#[derive(Debug)]
pub struct CommandFactory;

impl CommandFactory {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(tokens: &Vec<String>) -> Result<Box<dyn Command>, RequestError> {
        let command = tokens[0].to_lowercase();
        let body = tokens[1..tokens.len()].into();
        match CommandType::from_str(&command) {
            Ok(c) => match c {
                CommandType::Ping => match PingCommand::new(body) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
                CommandType::String(v) => handle_string_command(v, body),
                CommandType::List(v) => handle_list_command(v, body),
                CommandType::Set(v) => handle_set_command(v, body),
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
        StringCommandType::Set => match string::SetCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        StringCommandType::Get => match string::GetCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        StringCommandType::Incr => {
            match string::IncrCommand::new(body, string::NumOperator::Incr) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        StringCommandType::Decr => {
            match string::IncrCommand::new(body, string::NumOperator::Decr) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        StringCommandType::IncrBy => {
            match string::IncrbyCommand::new(body, string::NumOperator::Incr) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        StringCommandType::DecrBy => {
            match string::IncrbyCommand::new(body, string::NumOperator::Decr) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        StringCommandType::MGet => match string::MgetCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        StringCommandType::MSet => match string::MsetCommand::new(body) {
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
        ListCommandType::LPush => {
            match list::PushCommand::new(body, list::OperationDirection::Left) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        ListCommandType::LPop => {
            match list::PopCommand::new(body, list::OperationDirection::Left) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        ListCommandType::LRange => match list::LRangeCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        ListCommandType::LLen => match list::LLenCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        ListCommandType::RPush => {
            match list::PushCommand::new(body, list::OperationDirection::Right) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
        ListCommandType::RPop => {
            match list::PopCommand::new(body, list::OperationDirection::Right) {
                Ok(v) => Ok(v),
                Err(e) => Err(e),
            }
        }
    }
}

fn handle_set_command(
    v: SetCommandType,
    body: Vec<String>,
) -> Result<Box<dyn Command>, RequestError> {
    match v {
        SetCommandType::Add => match set::SAddCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        SetCommandType::Rem => match set::SRemCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        SetCommandType::Members => match set::SMembersCommand::new(body) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
    }
}
