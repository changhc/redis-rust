use crate::execution_result::{ExecutionResult, ResultType};

pub struct LpopResult {
    pub value: usize,
}

impl ExecutionResult for LpopResult {
    fn get_result_type(&self) -> ResultType {
        ResultType::Integer
    }
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
