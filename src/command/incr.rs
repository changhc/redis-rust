use crate::command::Command;
use crate::error::IncrCommandError;
use crate::execution_result::{ExecutionResult, IncrResult};
use std::collections::HashMap;

#[derive(Debug)]
pub struct IncrCommand {
    key: String,
}

impl IncrCommand {
    pub fn new(tokens: Vec<String>) -> Result<Self, IncrCommandError> {
        if tokens.len() != 1 {
            return Err(IncrCommandError::InvalidBody(format!(
                "Expected number of tokens: {}, received: {}",
                1,
                tokens.len()
            )));
        }
        Ok(IncrCommand {
            key: tokens[0].clone(),
        })
    }
}

impl Command for IncrCommand {
    fn execute(&self, data_store: &mut HashMap<String, String>) -> Box<dyn ExecutionResult> {
        let curr_value = data_store.get(&self.key).unwrap_or(&"0".to_string());
        match curr_value.parse::<i64>() {
            Ok(v) => match v.checked_add(1) {
                Some(updated) => {
                    data_store.insert(self.key.clone(), updated.to_string());
                    Box::new(IncrResult { value: updated })
                }
                None => Err(IncrCommandError::InvalidValue),
            },
            Err(_) => Err(IncrCommandError::InvalidValue),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::command::Command;

    use super::IncrCommand;

    #[test]
    fn should_accept_exactly_two_tokens() {
        match IncrCommand::new(vec!["foo".to_string(), "bar".to_string()]) {
            Ok(_) => panic!("should not be ok"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    "invalid command body. Details: Expected number of tokens: 1, received: 2"
                        .to_string()
                );
            }
        }
        match IncrCommand::new(vec!["foo".to_string()]) {
            Ok(v) => {
                assert_eq!(v.key, "foo".to_string());
            }
            Err(_) => panic!("should be ok"),
        }
    }

    #[test]
    fn should_insert_value_when_key_is_not_set() {
        let cmd = IncrCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = HashMap::<String, String>::new();
        assert!(ds.get(&"foo".to_string()).is_none());
        cmd.execute(&mut ds);
        assert_eq!(ds.get(&"foo".to_string()).unwrap(), &"1".to_string());
    }

    #[test]
    fn should_throw_error_when_value_is_not_int() {
        let cmd = IncrCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = HashMap::<String, String>::new();
        ds.insert("foo".to_string(), "bar".to_string()).unwrap();
        assert!(cmd.execute(&mut ds).is_err());
    }

    #[test]
    fn should_throw_error_when_value_overflows() {
        let cmd = IncrCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = HashMap::<String, String>::new();
        ds.insert("foo".to_string(), i64::MAX.to_string()).unwrap();
        assert!(cmd.execute(&mut ds).is_err());
    }
}
