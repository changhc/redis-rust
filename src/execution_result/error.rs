use super::{ExecutionResult, ResultType};

pub struct ErrorResult {
    pub message: String,
}

impl ExecutionResult for ErrorResult {
    fn get_result_type(&self) -> super::ResultType {
        ResultType::SimpleError
    }
    fn to_string(&self) -> String {
        format!("ERR {}", self.message).to_string()
    }
}
