use crate::command::Command;
use crate::error::GetCommandError;
use crate::execution_result::{ExecutionResult, GetResult};
use std::collections::HashMap;

#[derive(Debug)]
pub struct GetCommand {
    key: String,
}

impl GetCommand {
    pub fn new(tokens: Vec<String>) -> Result<Self, GetCommandError> {
        if tokens.len() != 1 {
            return Err(GetCommandError::InvalidBody(format!(
                "Expected number of tokens: {}, received: {}",
                1,
                tokens.len()
            )));
        }
        Ok(GetCommand {
            key: tokens[0].clone(),
        })
    }
}

impl Command for GetCommand {
    fn execute(
        &self,
        data_store: &mut HashMap<String, String>,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        let value = data_store.get(&self.key);
        Ok(Box::new(GetResult {
            value: match value {
                Some(v) => Some(v.clone()),
                None => None,
            },
        }))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::command::Command;

    use super::GetCommand;

    #[test]
    fn should_accept_exactly_one_token() {
        match GetCommand::new(vec!["foo".to_string()]) {
            Ok(v) => {
                assert_eq!(v.key, "foo".to_string());
            }
            Err(_) => panic!("should be ok"),
        }
        match GetCommand::new(vec!["foo".to_string(), "bar".to_string()]) {
            Ok(_) => panic!("should not be ok"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    "invalid command body. Details: Expected number of tokens: 1, received: 2"
                        .to_string()
                );
            }
        }
    }

    #[test]
    fn should_get_value_if_key_exists() {
        let cmd = GetCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = HashMap::<String, String>::new();
        ds.insert("foo".to_string(), "bar".to_string());
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "bar".to_string());
    }

    #[test]
    fn should_return_null_if_key_does_not_exist() {
        let cmd = GetCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = HashMap::<String, String>::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
    }
}
