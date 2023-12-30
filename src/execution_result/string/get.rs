use crate::execution_result::{to_null, to_simple_string, ExecutionResult};

pub struct GetResult {
    pub value: Option<String>,
}

impl ExecutionResult for GetResult {
    fn to_string(&self) -> String {
        match &self.value {
            Some(v) => v.clone(),
            None => "".to_string(),
        }
    }
    fn serialise(&self) -> String {
        match &self.value {
            Some(v) => to_simple_string(v),
            None => to_null(),
        }
    }
}
