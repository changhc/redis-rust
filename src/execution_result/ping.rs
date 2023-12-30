pub struct PingResult;
use crate::execution_result::{ExecutionResult, RespReply, SimpleStringReply};

impl ExecutionResult for PingResult {
    fn to_string(&self) -> String {
        "OK".to_string()
    }
    fn serialise(&self) -> String {
        SimpleStringReply {
            value: "OK".to_string(),
        }
        .serialise()
    }
}
