use crate::execution_result::{BulkStringResult, ExecutionResult, ResultType};

pub struct LpopResult {
    pub values: Vec<String>,
}

impl ExecutionResult for LpopResult {
    fn get_result_type(&self) -> ResultType {
        match self.values.len() {
            0 => ResultType::Null,
            1 => ResultType::SimpleString,
            _ => ResultType::Array,
        }
    }
    fn to_string(&self) -> String {
        match self.values.len() {
            0 => "".to_string(),
            1 => self.values[0].clone(),
            _ => format!(
                "{}\r\n{}",
                self.values.len(),
                self.values
                    .iter()
                    .map(|v| BulkStringResult { value: v.clone() }.serialise())
                    .collect::<Vec<_>>()
                    .join("")
            ),
        }
    }
}
