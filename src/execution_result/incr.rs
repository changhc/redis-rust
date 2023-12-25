use super::{ExecutionResult, ResultType};

pub struct IncrResult {
    pub value: i64,
}

impl ExecutionResult for IncrResult {
    fn get_result_type(&self) -> super::ResultType {
        ResultType::Integer
    }
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
