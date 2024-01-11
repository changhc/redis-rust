use crate::execution_result::{ArrayReply, BulkStringReply, ExecutionResult, RespReply};

pub struct LRangeResult {
    pub values: Vec<String>,
}

impl ExecutionResult for LRangeResult {
    fn to_string(&self) -> String {
        self.values.join(",")
    }
    fn serialise(&self) -> String {
        let mut rs: Vec<Box<dyn RespReply>> = Vec::new();
        for vv in &self.values {
            rs.push(Box::new(BulkStringReply { value: vv.clone() }))
        }
        ArrayReply { values: rs }.serialise()
    }
}
