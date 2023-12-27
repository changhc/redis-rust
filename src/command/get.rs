use crate::command::Command;
use crate::data_store::{DataStore, RedisEntryType};
use crate::error::{ExecutionError, RequestError, InternalError};
use crate::execution_result::{ExecutionResult, GetResult};

#[derive(Debug)]
pub struct GetCommand {
    key: String,
}

impl GetCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 1 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(GetCommand {
            key: tokens[0].clone(),
        }))
    }
}

impl Command for GetCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        let value = data_store.get(&self.key);
        Ok(Box::new(GetResult {
            value: match value {
                Some(entry) => match entry.type_ {
                    RedisEntryType::String => match &entry.string {
                        Some(v) => Some(v.clone()),
                        None => {
                            log::error!("Integration error at key '{}': expecting type 'String' but data is not found", &self.key);
                            return Err(Box::new(InternalError::Error));
                        }
                    },
                    _ => return Err(Box::new(ExecutionError::IncorrectType)),
                }
                None => None,
            },
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::data_store::{DataStore, RedisEntry};

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
                    "ERR wrong number of arguments for command".to_string()
                );
            }
        }
    }

    #[test]
    fn should_get_value_if_key_exists() {
        let cmd = GetCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = DataStore::new();
        ds.insert(
            "foo".to_string(),
            RedisEntry::create_string(&"bar".to_string()),
        );
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "bar".to_string());
    }

    #[test]
    fn should_return_null_if_key_does_not_exist() {
        let cmd = GetCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
    }
}
