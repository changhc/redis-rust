use crate::execution_result::{ArrayReply, BulkStringReply, ExecutionResult, RespReply};

pub struct SMembersResult {
    pub values: Vec<String>,
}

impl ExecutionResult for SMembersResult {
    fn to_string(&self) -> String {
        self.values.to_vec().join(",")
    }
    fn serialise(&self) -> String {
        let mut replies: Vec<Box<dyn RespReply>> = Vec::new();
        for value in &self.values {
            replies.push(Box::new(BulkStringReply {
                value: value.clone(),
            }));
        }
        ArrayReply { values: replies }.serialise()
    }
}
