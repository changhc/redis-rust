use std::collections::HashSet;

use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::set::SDiffResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct SDiffCommand {
    keys: Vec<String>,
}

impl SDiffCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.is_empty() {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(SDiffCommand { keys: tokens }))
    }
}

impl Command for SDiffCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        let values = match data_store.get_set_mut(&self.keys[0])? {
            Some(set) => {
                let mut result: HashSet<String> = set.iter().cloned().collect();
                for key in &self.keys[1..] {
                    if let Some(right_set) = data_store.get_set_mut(key)? {
                        for v in right_set.iter() {
                            result.remove(v);
                        }
                    }
                }
                result
            }
            None => HashSet::<String>::new(),
        }
        .iter()
        .cloned()
        .collect::<Vec<String>>();
        Ok(Box::new(SDiffResult { values }))
    }
}

#[cfg(test)]
mod test {
    use crate::command::set::{SAddCommand, SDiffCommand};
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_at_least_one_token() {
        let err = SDiffCommand::new(vec![]).err().unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
        let v = SDiffCommand::new(vec!["foo".to_string()]).unwrap();
        assert_eq!(v.keys, vec!["foo".to_string()]);
    }

    #[test]
    fn should_return_empty_set_if_first_key_does_not_exist() {
        let cmd = SDiffCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
    }

    #[test]
    fn should_return_diff_elements() {
        let key1 = "foo".to_string();
        let key2 = "bar".to_string();
        let mut ds = DataStore::new();
        SAddCommand::new(vec![key1.clone(), "v1".to_string(), "v2".to_string()])
            .unwrap()
            .execute(&mut ds)
            .unwrap();
        SAddCommand::new(vec![key2.clone(), "v2".to_string(), "v3".to_string()])
            .unwrap()
            .execute(&mut ds)
            .unwrap();
        let cmd = SDiffCommand::new(vec![key1.clone(), key2.clone()]).unwrap();
        let result = cmd.execute(&mut ds).unwrap().to_string();
        let mut values: Vec<&str> = result.split(',').collect::<Vec<&str>>();
        values.sort();
        assert_eq!(values, vec!["v1"]);
    }
}
