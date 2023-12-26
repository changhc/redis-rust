use crate::command::Command;
use crate::error::RequestError;
use crate::execution_result::{ExecutionResult, SetResult};
use std::collections::HashMap;

#[derive(Debug)]
pub struct SetCommand {
    key: String,
    value: String,
}

impl SetCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 2 {
            return Err(RequestError::InvalidCommandBody(format!(
                "Expected number of tokens: {}, received: {}",
                2,
                tokens.len()
            )));
        }
        Ok(Box::new(SetCommand {
            key: tokens[0].clone(),
            value: tokens[1].clone(),
        }))
    }
}

impl Command for SetCommand {
    fn execute(
        &self,
        data_store: &mut HashMap<String, String>,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        data_store.insert(self.key.clone(), self.value.clone());
        Ok(Box::new(SetResult {}))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::command::Command;

    use super::SetCommand;

    #[test]
    fn should_accept_exactly_two_tokens() {
        match SetCommand::new(vec!["foo".to_string()]) {
            Ok(_) => panic!("should not be ok"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    "invalid command body. Details: Expected number of tokens: 2, received: 1"
                        .to_string()
                );
            }
        }
        match SetCommand::new(vec![
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
        ]) {
            Ok(_) => panic!("should not be ok"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    "invalid command body. Details: Expected number of tokens: 2, received: 3"
                        .to_string()
                );
            }
        }
        match SetCommand::new(vec!["foo".to_string(), "bar".to_string()]) {
            Ok(v) => {
                assert_eq!(v.key, "foo".to_string());
                assert_eq!(v.value, "bar".to_string());
            }
            Err(_) => panic!("should be ok"),
        }
    }

    #[test]
    fn should_insert_value() {
        let cmd = SetCommand::new(vec!["foo".to_string(), "bar".to_string()]).unwrap();
        let mut ds = HashMap::<String, String>::new();
        assert!(ds.get(&"foo".to_string()).is_none());
        cmd.execute(&mut ds).unwrap();
        assert_eq!(ds.get(&"foo".to_string()).unwrap(), &"bar".to_string());
    }
}
