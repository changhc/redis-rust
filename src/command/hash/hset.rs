use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::hash::HSetResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct HSetCommand {
    key: String,
    values: Vec<(String, String)>,
}

impl HSetCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() % 2 != 1 || tokens.len() < 3 {
            return Err(RequestError::IncorrectArgCount);
        }
        let mut values = Vec::new();
        for i in 0..tokens.len() / 2 {
            values.push((tokens[2 * i + 1].clone(), tokens[2 * i + 2].clone()));
        }
        Ok(Box::new(HSetCommand {
            key: tokens[0].clone(),
            values,
        }))
    }
}

impl Command for HSetCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        // TODO: atomicity
        let hash = match data_store.get_hash_mut(&self.key)? {
            Some(hash) => hash,
            None => {
                let _ = data_store.insert_hash(&self.key);
                data_store.get_hash_mut(&self.key).unwrap().unwrap()
            }
        };
        let mut count = 0;
        for (key, value) in &self.values {
            count += match hash.insert(key.clone(), value.clone()) {
                Some(_) => 0,
                None => 1,
            }
        }
        Ok(Box::new(HSetResult { value: count }))
    }
}

#[cfg(test)]
mod test {
    mod test_set {
        use crate::command::hash::HSetCommand;
        use crate::command::Command;
        use crate::data_store::DataStore;

        #[test]
        fn should_accept_correct_amount_of_tokens() {
            let err = HSetCommand::new(vec!["foo".to_string()]).err().unwrap();
            assert_eq!(
                err.to_string(),
                "ERR wrong number of arguments for command".to_string()
            );
            let err = HSetCommand::new(vec!["foo".to_string(), "bar".to_string()])
                .err()
                .unwrap();
            assert_eq!(
                err.to_string(),
                "ERR wrong number of arguments for command".to_string()
            );
            let v = HSetCommand::new(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string(),
            ])
            .unwrap();
            assert_eq!(v.key, "foo".to_string());
            assert_eq!(v.values, vec![("bar".to_string(), "baz".to_string())]);
        }

        #[test]
        fn should_insert_value() {
            let mut ds = DataStore::new();
            let key = "foo".to_string();
            let cmd = HSetCommand::new(vec![
                key.clone(),
                "k1".to_string(),
                "v1".to_string(),
                "k2".to_string(),
                "v2".to_string(),
            ])
            .unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "2".to_string());
            let hash = ds.get_hash_mut(&key).unwrap().unwrap();
            assert_eq!(hash.len(), 2);
            assert_eq!(hash.get(&"k1".to_string()).unwrap(), &"v1".to_string());
            assert_eq!(hash.get(&"k2".to_string()).unwrap(), &"v2".to_string());

            // Should return 0 because key "bar" already exists
            let cmd =
                HSetCommand::new(vec![key.clone(), "k1".to_string(), "v3".to_string()]).unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "0".to_string());
            let hash = ds.get_hash_mut(&key).unwrap().unwrap();
            assert_eq!(hash.get(&"k1".to_string()).unwrap(), &"v3".to_string());
        }
    }
}
