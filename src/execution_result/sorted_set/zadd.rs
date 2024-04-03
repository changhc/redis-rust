use crate::execution_result::{ExecutionResult, RespReply, UnsignedIntegerReply};

pub struct ZAddResult {
    pub value: u64,
}

impl ExecutionResult for ZAddResult {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
    fn serialise(&self) -> String {
        UnsignedIntegerReply { value: self.value }.serialise()
    }
}
