use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::{ExecutionResult, PingResult};

#[derive(Debug)]
pub struct PingCommand;

impl PingCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if !tokens.is_empty() {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(PingCommand {}))
    }
}

impl Command for PingCommand {
    fn execute(
        &self,
        _: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        Ok(Box::new(PingResult {}))
    }
}
