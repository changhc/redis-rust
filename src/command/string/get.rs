use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::{string::GetResult, ExecutionResult};

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
        match data_store.get_string(&self.key) {
            Ok(v) => Ok(Box::new(GetResult { value: v.cloned() })),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data_store::DataStore;

    use crate::command::Command;

    use super::GetCommand;

    #[test]
    fn should_accept_exactly_one_token() {
        let v = GetCommand::new(vec!["foo".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
        let err = GetCommand::new(vec!["foo".to_string(), "bar".to_string()])
            .err()
            .unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
    }

    #[test]
    fn should_get_value_if_key_exists() {
        let cmd = GetCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = DataStore::new();
        ds.set_string_overwrite(&"foo".to_string(), &"bar".to_string());
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
