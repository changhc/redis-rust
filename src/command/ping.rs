use crate::command::Command;
use crate::execution_result::{ExecutionResult, PingResult};
use std::collections::HashMap;

#[derive(Debug)]
pub struct PingCommand;

impl PingCommand {
    pub fn new(_: Vec<String>) -> Self {
        PingCommand {}
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
