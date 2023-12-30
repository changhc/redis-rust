use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::string::{MsetResult, SetResult};
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct SetCommand {
    key: String,
    value: String,
}

impl SetCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 2 {
            return Err(RequestError::IncorrectArgCount);
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
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        data_store.set_string_overwrite(&self.key, &self.value);
        Ok(Box::new(SetResult {}))
    }
}

#[derive(Debug)]
pub struct MsetCommand {
    pairs: Vec<(String, String)>,
}

impl MsetCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() % 2 != 0 {
            return Err(RequestError::IncorrectArgCount);
        }
        let mut pairs = Vec::new();
        for i in 0..tokens.len() / 2 {
            pairs.push((tokens[2 * i].clone(), tokens[2 * i + 1].clone()));
        }
        Ok(Box::new(MsetCommand { pairs: pairs }))
    }
}

impl Command for MsetCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        for p in &self.pairs {
            data_store.set_string_overwrite(&p.0, &p.1);
        }
        Ok(Box::new(MsetResult {}))
    }
}

#[cfg(test)]
mod test {
    mod test_set {
        use crate::command::string::SetCommand;
        use crate::command::Command;
        use crate::data_store::DataStore;

        #[test]
        fn should_accept_exactly_two_tokens() {
            let err = SetCommand::new(vec!["foo".to_string()]).err().unwrap();
            assert_eq!(
                err.to_string(),
                "ERR wrong number of arguments for command".to_string()
            );
            let err = SetCommand::new(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string(),
            ])
            .err()
            .unwrap();
            assert_eq!(
                err.to_string(),
                "ERR wrong number of arguments for command".to_string()
            );
            let v = SetCommand::new(vec!["foo".to_string(), "bar".to_string()]).unwrap();
            assert_eq!(v.key, "foo".to_string());
            assert_eq!(v.value, "bar".to_string());
        }

        #[test]
        fn should_insert_value() {
            let key = "foo".to_string();
            let cmd = SetCommand::new(vec![key.clone(), "bar".to_string()]).unwrap();
            let mut ds = DataStore::new();
            assert!(ds.get_string(&key).unwrap().is_none());
            cmd.execute(&mut ds).unwrap();
            assert_eq!(ds.get_string(&key).unwrap().unwrap(), &"bar".to_string());
        }
    }

    mod test_mset {
        use crate::command::string::MsetCommand;
        use crate::command::Command;
        use crate::data_store::DataStore;

        #[test]
        fn should_insert_values() {
            let cmd = MsetCommand::new(vec![
                "k1".to_string(),
                "v1".to_string(),
                "k2".to_string(),
                "v2".to_string(),
            ])
            .unwrap();
            let mut ds = DataStore::new();
            assert!(ds.get_string(&"k1".to_string()).unwrap().is_none());
            assert!(ds.get_string(&"k2".to_string()).unwrap().is_none());
            cmd.execute(&mut ds).unwrap();
            assert_eq!(
                ds.get_string(&"k1".to_string()).unwrap().unwrap(),
                &"v1".to_string()
            );
            assert_eq!(
                ds.get_string(&"k2".to_string()).unwrap().unwrap(),
                &"v2".to_string()
            );
        }
    }
}
