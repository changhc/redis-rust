pub struct PingResult;
use crate::execution_result::{to_simple_string, ExecutionResult};

impl ExecutionResult for PingResult {
    fn to_string(&self) -> String {
        "OK".to_string()
    }
    fn serialise(&self) -> String {
        to_simple_string(&self.to_string())
    }
}
