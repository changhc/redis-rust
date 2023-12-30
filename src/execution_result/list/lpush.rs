use crate::execution_result::{to_integer, ExecutionResult};

pub struct LpushResult {
    pub value: usize,
}

impl ExecutionResult for LpushResult {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
    fn serialise(&self) -> String {
        to_integer(&self.to_string())
    }
}
