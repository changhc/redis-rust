use crate::execution_result::{ExecutionResult, ResultType};

pub struct LpushResult;

impl ExecutionResult for LpushResult {
    fn get_result_type(&self) -> ResultType {
        ResultType::SimpleString
    }
    fn to_string(&self) -> String {
        "OK".to_string()
    }
}
