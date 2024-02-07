use crate::execution_result::{ExecutionResult, IntegerReply, RespReply};

pub struct SIsmemberResult {
    pub value: usize,
}

impl ExecutionResult for SIsmemberResult {
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
