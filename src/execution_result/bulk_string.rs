use super::{ExecutionResult, ResultType};

pub struct BulkStringResult {
    pub value: String,
}

impl ExecutionResult for BulkStringResult {
    fn get_result_type(&self) -> super::ResultType {
        ResultType::BulkString
    }
    fn to_string(&self) -> String {
        format!("{}\r\n{}", self.value.len(), self.value)
    }
}
