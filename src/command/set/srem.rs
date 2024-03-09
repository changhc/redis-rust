use std::collections::HashSet;

use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::set::SRemResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct SRemCommand {
    key: String,
    values: HashSet<String>,
}

impl SRemCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() < 2 {
            return Err(RequestError::IncorrectArgCount);
        }
        let mut set = HashSet::new();
        for t in &tokens[1..] {
            set.insert(t.clone());
        }
        Ok(Box::new(SRemCommand {
            key: tokens[0].clone(),
            values: set,
        }))
    }
}

impl Command for SRemCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        let count = match data_store.get_set_mut(&self.key)? {
            Some(set) => {
                let mut count = 0;
                for value in &self.values {
                    count += set.remove(value) as usize;
                }
                count
            }
            None => 0,
        };
        Ok(Box::new(SRemResult { value: count }))
    }
}

#[cfg(test)]
mod test {
    use crate::command::set::{SAddCommand, SRemCommand};
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_at_least_two_tokens() {
        let err = SRemCommand::new(vec!["foo".to_string()]).err().unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
        let v = SRemCommand::new(vec![
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
        ])
        .unwrap();
        assert_eq!(v.key, "foo".to_string());
        assert_eq!(v.values.len(), 2);
        assert!(v.values.contains(&"bar".to_string()));
        assert!(v.values.contains(&"baz".to_string()));
    }

    #[test]
    fn should_throw_error_for_non_set_keys() {
        let key = "foo".to_string();
        let cmd =
            SRemCommand::new(vec![key.clone(), "bar".to_string(), "baz".to_string()]).unwrap();
        let mut ds = DataStore::new();
        ds.set_string(&key, "something").unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(
            result.err().unwrap().to_string(),
            "WRONGTYPE Operation against a key holding the wrong kind of value".to_string()
        );
    }

    #[test]
    fn should_return_0_if_key_does_not_exist() {
        let key = "foo".to_string();
        let cmd =
            SRemCommand::new(vec![key.clone(), "bar".to_string(), "baz".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "0".to_string());
    }

    #[test]
    fn should_remove_values() {
        let key = "foo".to_string();
        let mut ds = DataStore::new();
        // insert set
        SAddCommand::new(vec![key.clone(), "v1".to_string(), "v3".to_string()])
            .unwrap()
            .execute(&mut ds)
            .unwrap();

        let cmd = SRemCommand::new(vec![key.clone(), "v1".to_string(), "v2".to_string()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "1".to_string());
        let set = ds.get_set_mut(&key).unwrap().unwrap();
        assert!(set.contains(&"v3".to_string()));
    }
}
