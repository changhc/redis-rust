use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::string::MgetResult;
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

#[derive(Debug)]
pub struct MgetCommand {
    keys: Vec<String>,
}

impl MgetCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.is_empty() {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(MgetCommand { keys: tokens }))
    }
}

impl Command for MgetCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        let mut res = Vec::new();
        for key in &self.keys {
            match data_store.get_string(key) {
                Ok(v) => res.push(v.cloned()),
                Err(e) => return Err(e),
            }
        }
        Ok(Box::new(MgetResult { values: res }))
    }
}

#[cfg(test)]
mod test {
    mod test_get {
        use crate::command::string::GetCommand;
        use crate::command::Command;
        use crate::data_store::DataStore;

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
    mod test_mget {
        use crate::command::string::MgetCommand;
        use crate::command::Command;
        use crate::data_store::DataStore;

        #[test]
        fn should_get_values_if_keys_exist() {
            let cmd = MgetCommand::new(vec!["k1".to_string(), "k2".to_string(), "k3".to_string()])
                .unwrap();
            let mut ds = DataStore::new();
            ds.set_string_overwrite(&"k1".to_string(), &"v1".to_string());
            ds.set_string_overwrite(&"k3".to_string(), &"v3".to_string());
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "v1,,v3".to_string());
        }
    }
}
