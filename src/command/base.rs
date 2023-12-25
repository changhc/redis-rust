use crate::execution_result::ExecutionResult;
use std::collections::HashMap;

pub trait Command {
    fn execute(
        &self,
        data_store: &mut HashMap<String, String>,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>>;
}
