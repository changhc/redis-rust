use super::ResultType;

pub trait ExecutionResult {
    fn get_result_type(&self) -> ResultType;
    fn to_string(&self) -> String;
    fn serialise(&self) -> String {
        format!(
            "{}{}\r\n",
            self.get_result_type().to_string(),
            self.to_string()
        )
    }
}
