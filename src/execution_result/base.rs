pub trait ExecutionResult {
    fn to_string(&self) -> String;
    fn serialise(&self) -> String;
}
