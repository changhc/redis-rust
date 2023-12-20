use crate::error::RequestError;
use std::fmt::Debug;
use std::str::FromStr;

pub enum CommandType {
    PING,
}

impl FromStr for CommandType {
    type Err = ();

    fn from_str(s: &str) -> Result<CommandType, Self::Err> {
        match s {
            "PING" => Ok(CommandType::PING),
            _ => Err(()),
        }
    }
}

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

#[derive(Debug)]
pub struct PingCommand;

impl PingCommand {
    pub fn new(_: Vec<String>) -> Self {
        PingCommand {}
    }
}

impl Command for PingCommand {
    fn execute(&self) -> Box<dyn ExecutionResult> {
        Box::new(PingResult {})
    }
}

pub struct PingResult;

impl ExecutionResult for PingResult {
    fn to_string(&self) -> String {
        "+PONG\r\n".to_string()
    }
}

pub trait ExecutionResult {
    fn to_string(&self) -> String;
}

pub trait Command {
    fn execute(&self) -> Box<dyn ExecutionResult>;
}
