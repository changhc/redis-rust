use crate::execution_result::{ExecutionResult, RespReply, SimpleStringReply};

pub struct SetResult;

impl ExecutionResult for SetResult {
    fn to_string(&self) -> String {
        "OK".to_string()
    }
    fn serialise(&self) -> String {
        SimpleStringReply {
            value: self.to_string(),
        }
        .serialise()
    }
}
