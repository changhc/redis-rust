use std::collections::HashSet;

use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::set::SAddResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct SAddCommand {
    key: String,
    values: HashSet<String>,
}

impl SAddCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() < 2 {
            return Err(RequestError::IncorrectArgCount);
        }
        let mut set = HashSet::new();
        for t in &tokens[1..] {
            set.insert(t.clone());
        }
        Ok(Box::new(SAddCommand {
            key: tokens[0].clone(),
            values: set,
        }))
    }
}

impl Command for SAddCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_set_mut(&self.key) {
            Ok(set_op) => {
                let set = match set_op {
                    Some(set) => set,
                    None => {
                        let _ = data_store.insert_set(&self.key);
                        data_store.get_set_mut(&self.key).unwrap().unwrap()
                    }
                };
                let mut count = 0;
                for value in &self.values {
                    count += set.insert(value.clone()) as usize;
                }
                Ok(Box::new(SAddResult { value: count }))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command::set::SAddCommand;
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_at_least_two_tokens() {
        let err = SAddCommand::new(vec!["foo".to_string()]).err().unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
        let v = SAddCommand::new(vec![
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
    fn should_add_item_if_key_does_not_exist() {
        let key = "foo".to_string();
        let cmd =
            SAddCommand::new(vec![key.clone(), "bar".to_string(), "baz".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "2".to_string());
        let set = ds.get_set_mut(&key).unwrap().unwrap();
        assert_eq!(set.len(), 2);
        assert!(set.contains(&"baz".to_string()));
        assert!(set.contains(&"bar".to_string()));
    }

    #[test]
    fn should_not_add_duplicates() {
        let key = "foo".to_string();
        let cmd = SAddCommand::new(vec![key.clone(), "bar".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "1".to_string());
        let set = ds.get_set_mut(&key).unwrap().unwrap();
        assert!(set.contains(&"bar".to_string()));
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "0".to_string());
    }
}
