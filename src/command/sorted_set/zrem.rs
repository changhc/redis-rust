use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::sorted_set::ZRemResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct ZRemCommand {
    key: String,
    values: Vec<String>,
}

impl ZRemCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() < 2 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(ZRemCommand {
            key: tokens[0].clone(),
            values: tokens[1..].to_vec(),
        }))
    }
}

impl Command for ZRemCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        // TODO: atomicity
        let count = match data_store.get_sorted_set_mut(&self.key)? {
            Some(sorted_set) => {
                let mut count = 0;
                for element in &self.values {
                    count += sorted_set.remove(element) as u64;
                }
                count
            }
            None => 0,
        };

        Ok(Box::new(ZRemResult { value: count }))
    }
}

#[cfg(test)]
mod test {
    mod test_set {
        use crate::command::sorted_set::{ZAddCommand, ZRemCommand};
        use crate::command::Command;
        use crate::data_store::DataStore;

        #[test]
        fn should_accept_correct_amount_of_tokens() {
            let err = ZRemCommand::new(vec!["foo".to_string()]).err().unwrap();
            assert_eq!(
                err.to_string(),
                "ERR wrong number of arguments for command".to_string()
            );
            let v = ZRemCommand::new(vec!["foo".to_string(), "baz".to_string()]).unwrap();
            assert_eq!(v.key, "foo".to_string());
            assert_eq!(v.values, vec!["baz".to_string()]);
        }

        #[test]
        fn should_remove_value() {
            let mut ds = DataStore::new();
            let key = "foo".to_string();
            ZAddCommand::new(vec![
                key.clone(),
                "2.0".to_string(),
                "v1".to_string(),
                "1.0".to_string(),
                "v2".to_string(),
            ])
            .unwrap()
            .execute(&mut ds)
            .unwrap();

            let cmd = ZRemCommand::new(vec![key.clone(), "v1".to_string()]).unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "1".to_string());
            let sorted_set = ds.get_sorted_set_mut(&key).unwrap().unwrap();
            assert!(sorted_set.get("v1").is_none());

            // Should return 0 because key "v1" is already gone
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "0".to_string());

            // v3 does not exist
            let cmd = ZRemCommand::new(vec![key.clone(), "v3".to_string()]).unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "0".to_string());
        }
    }
}
