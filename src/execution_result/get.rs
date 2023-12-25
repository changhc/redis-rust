use super::{ExecutionResult, ResultType};

pub struct GetResult {
    pub value: Option<String>,
}

impl ExecutionResult for GetResult {
    fn get_result_type(&self) -> super::ResultType {
        match self.value {
            Some(_) => ResultType::SimpleString,
            None => ResultType::Null,
        }
    }
    fn to_string(&self) -> String {
        match &self.value {
            Some(v) => v.clone(),
            None => "".to_string(),
        }
    }
}
