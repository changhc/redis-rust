use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::{ConfigGetResult, ExecutionResult};

#[derive(Debug)]
pub struct ConfigGetCommand;

impl ConfigGetCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        Ok(Box::new(ConfigGetCommand {}))
    }
}

impl Command for ConfigGetCommand {
    fn execute(
        &self,
        _: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        Ok(Box::new(ConfigGetResult {}))
    }
}
