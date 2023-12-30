use crate::execution_result::{ExecutionResult, RespReply, SimpleErrorReply};

pub struct ErrorResult {
    pub message: String,
}

impl ExecutionResult for ErrorResult {
    fn to_string(&self) -> String {
        self.message.clone()
    }
    fn serialise(&self) -> String {
        SimpleErrorReply {
            message: self.to_string(),
        }
        .serialise()
    }
}
