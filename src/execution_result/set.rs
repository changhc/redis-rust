pub struct SetResult;
use super::{ExecutionResult, ResultType};

impl ExecutionResult for SetResult {
    fn get_result_type(&self) -> super::ResultType {
        ResultType::SimpleString
    }
    fn to_string(&self) -> String {
        "OK".to_string()
    }
}
