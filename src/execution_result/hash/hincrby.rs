use crate::execution_result::{ExecutionResult, IntegerReply, RespReply};

pub struct HIncrByResult {
    pub value: i64,
}

impl ExecutionResult for HIncrByResult {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
    fn serialise(&self) -> String {
        IntegerReply { value: self.value }.serialise()
    }
}
