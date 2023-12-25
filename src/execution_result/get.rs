use super::{ExecutionResult, ResultType};

pub struct GetResult {
    pub value: String,
}

impl ExecutionResult for GetResult {
    fn get_result_type(&self) -> super::ResultType {
        ResultType::SimpleString
    }
    fn to_string(&self) -> String {
        self.value.clone()
    }
}
