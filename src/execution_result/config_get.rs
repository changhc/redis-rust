pub struct ConfigGetResult;
use std::collections::HashMap;

use crate::execution_result::{ExecutionResult, MapReply, RespReply};

impl ExecutionResult for ConfigGetResult {
    fn to_string(&self) -> String {
        "OK".to_string()
    }
    fn serialise(&self) -> String {
        MapReply {
            values: HashMap::new(),
        }
        .serialise()
    }
}
