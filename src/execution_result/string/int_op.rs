use crate::execution_result::{ExecutionResult, IntegerReply, RespReply};

pub struct IntOpResult {
    pub value: i64,
}

impl ExecutionResult for IntOpResult {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
    fn serialise(&self) -> String {
        IntegerReply { value: self.value }.serialise()
    }
}
