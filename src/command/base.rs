use crate::execution_result::ExecutionResult;

pub trait Command {
    fn execute(&self) -> Box<dyn ExecutionResult>;
}
