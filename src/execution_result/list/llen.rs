use crate::execution_result::{ExecutionResult, IntegerReply, RespReply};

pub struct LLenResult {
    pub value: usize,
}

impl ExecutionResult for LLenResult {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
    fn serialise(&self) -> String {
        IntegerReply {
            value: self.value as i64,
        }
        .serialise()
    }
}
