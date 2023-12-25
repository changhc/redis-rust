use crate::command::Command;
use crate::error::RequestError;
use crate::execution_result::{ExecutionResult, PingResult};
use std::collections::HashMap;

#[derive(Debug)]
pub struct PingCommand;

impl PingCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 0 {
            return Err(RequestError::InvalidCommandBody(format!(
                "Expected number of tokens: {}, received: {}",
                1,
                tokens.len()
            )));
        }
        Ok(Box::new(PingCommand {}))
    }
}

impl Command for PingCommand {
    fn execute(
        &self,
        _: &mut HashMap<String, String>,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        Ok(Box::new(PingResult {}))
    }
}
