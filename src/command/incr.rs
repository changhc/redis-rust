use crate::command::Command;
use crate::error::IncrCommandError;
use crate::execution_result::{ExecutionResult, IncrResult};
use std::collections::HashMap;

#[derive(Debug)]
struct NumOp {
    value: i64,
}

impl NumOp {
    pub fn new(value: &i64) -> Self {
        Self { value: *value }
    }

    pub fn execute(
        &self,
        key: &String,
        data_store: &mut HashMap<String, String>,
    ) -> Result<i64, ()> {
        let default = "0".to_string();
        let curr_value = data_store.get(key).unwrap_or(&default);
        match curr_value.parse::<i64>() {
            Ok(v) => match v.checked_add(self.value) {
                Some(updated) => {
                    data_store.insert(key.clone(), updated.to_string());
                    Ok(updated)
                }
                None => Err(()),
            },
            Err(_) => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct IncrCommand {
    key: String,
    op: NumOp,
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
            op: NumOp::new(&1),
        })
    }
}

impl Command for IncrCommand {
    fn execute(
        &self,
        data_store: &mut HashMap<String, String>,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match self.op.execute(&self.key, data_store) {
            Ok(v) => Ok(Box::new(IncrResult { value: v })),
            Err(_) => Err(Box::new(IncrCommandError::InvalidValue)),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::command::Command;

    use super::IncrCommand;
    use crate::error::IncrCommandError;

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
        cmd.execute(&mut ds).unwrap();
        assert_eq!(ds.get(&"foo".to_string()).unwrap(), &"1".to_string());
    }

    #[test]
    fn should_throw_error_when_value_is_not_int() {
        let cmd = IncrCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = HashMap::<String, String>::new();
        ds.insert("foo".to_string(), "bar".to_string());
        assert!(cmd.execute(&mut ds).is_err());
    }

    #[test]
    fn should_throw_error_when_value_overflows() {
        let cmd = IncrCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = HashMap::<String, String>::new();
        ds.insert("foo".to_string(), i64::MAX.to_string());
        match cmd.execute(&mut ds) {
            Ok(_) => panic!("should not be ok"),
            Err(e) => assert_eq!(e.to_string(), IncrCommandError::InvalidValue.to_string()),
        }
    }
}
