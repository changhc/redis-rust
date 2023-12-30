use crate::execution_result::{to_array, to_null, to_simple_string, ExecutionResult};

pub struct LpopResult {
    pub values: Vec<String>,
}

impl ExecutionResult for LpopResult {
    fn to_string(&self) -> String {
        self.values.join(",")
    }
    fn serialise(&self) -> String {
        match self.values.len() {
            0 => to_null(),
            1 => to_simple_string(&self.values[0]),
            _ => to_array(&self.values),
        }
    }
}
