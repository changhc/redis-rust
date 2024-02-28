use crate::execution_result::{ExecutionResult, NullReply, RespReply, SimpleStringReply};

pub struct HGetResult {
    pub value: Option<String>,
}

impl ExecutionResult for HGetResult {
    fn to_string(&self) -> String {
        match &self.value {
            Some(v) => v.clone(),
            None => "".to_string(),
        }
    }
    fn serialise(&self) -> String {
        match &self.value {
            Some(v) => SimpleStringReply { value: v.clone() }.serialise(),
            None => NullReply {}.serialise(),
        }
    }
}
