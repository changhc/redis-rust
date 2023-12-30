use crate::execution_result::{to_integer, ExecutionResult};

pub struct IntOpResult {
    pub value: i64,
}

impl ExecutionResult for IntOpResult {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
    fn serialise(&self) -> String {
        to_integer(&self.to_string())
    }
}
