use crate::execution_result::{ExecutionResult, ResultType};

pub struct SetResult;

impl ExecutionResult for SetResult {
    fn get_result_type(&self) -> ResultType {
        ResultType::SimpleString
    }
    fn to_string(&self) -> String {
        "OK".to_string()
    }
}
