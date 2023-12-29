use crate::execution_result::{ExecutionResult, ResultType};

pub struct IntOpResult {
    pub value: i64,
}

impl ExecutionResult for IntOpResult {
    fn get_result_type(&self) -> ResultType {
        ResultType::Integer
    }
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
