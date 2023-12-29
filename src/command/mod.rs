mod base;
mod get;
mod ping;
use super::error::RequestError;
pub use base::Command;
use get::GetCommand;
use ping::PingCommand;
mod set;
use set::SetCommand;
mod int_op;
use int_op::{IncrCommand, IncrbyCommand, NumOperator};
mod types;
use std::str::FromStr;
use types::CommandType;

mod list;

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
                CommandType::SET => match SetCommand::new(body) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
                CommandType::GET => match GetCommand::new(body) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
                CommandType::INCR => match IncrCommand::new(body, NumOperator::INCR) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
                CommandType::DECR => match IncrCommand::new(body, NumOperator::DECR) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
                CommandType::INCRBY => match IncrbyCommand::new(body, NumOperator::INCR) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
                CommandType::DECRBY => match IncrbyCommand::new(body, NumOperator::DECR) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
                CommandType::LPUSH => match list::LpushCommand::new(body) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
                CommandType::LPOP => match list::LpopCommand::new(body) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e),
                },
            },
            Err(_) => Err(RequestError::UnsupportedCommand(command)),
        }
    }
}
