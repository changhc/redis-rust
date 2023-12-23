use crate::command::Command;
use crate::error::SetCommandError;
use crate::execution_result::{ExecutionResult, SetResult};

#[derive(Debug)]
pub struct SetCommand {
    key: String,
    value: String,
}

impl SetCommand {
    pub fn new(tokens: Vec<String>) -> Result<Self, SetCommandError> {
        if tokens.len() != 2 {
            return Err(SetCommandError::InvalidBody(format!(
                "Expected number of tokens: {}, received: {}",
                2,
                tokens.len()
            )));
        }
        Ok(SetCommand {
            key: tokens[0].clone(),
            value: tokens[0].clone(),
        })
    }
}

impl Command for SetCommand {
    fn execute(&self) -> Box<dyn ExecutionResult> {
        Box::new(SetResult {})
    }
}