use crate::execution_result::{ExecutionResult, NullReply, RespReply, UnsignedIntegerReply};

pub struct ZRankResult {
    pub value: Option<u64>,
}

impl ExecutionResult for ZRankResult {
    fn to_string(&self) -> String {
        match &self.value {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }
    fn serialise(&self) -> String {
        match &self.value {
            Some(v) => UnsignedIntegerReply { value: *v }.serialise(),
            None => NullReply {}.serialise(),
        }
    }
}
