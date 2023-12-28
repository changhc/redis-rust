use crate::data_store::DataStore;
use crate::execution_result::ExecutionResult;

pub trait Command {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>>;
}
