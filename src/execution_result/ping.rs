pub struct PingResult;
use super::{ExecutionResult, ResultType};

impl ExecutionResult for PingResult {
    fn get_result_type(&self) -> super::ResultType {
        ResultType::SimpleString
    }
    fn to_string(&self) -> String {
        "PONG".to_string()
    }
}
