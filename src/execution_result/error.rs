use crate::execution_result::{to_simple_error, ExecutionResult};

pub struct ErrorResult {
    pub message: String,
}

impl ExecutionResult for ErrorResult {
    fn to_string(&self) -> String {
        self.message.clone()
    }
    fn serialise(&self) -> String {
        to_simple_error(&self.to_string())
    }
}
