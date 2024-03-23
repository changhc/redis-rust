use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::sorted_set::ZAddResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct ZAddCommand {
    key: String,
    values: Vec<(f64, String)>,
}

impl ZAddCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() % 2 != 1 || tokens.len() < 3 {
            return Err(RequestError::IncorrectArgCount);
        }
        let mut values = Vec::new();
        for i in 0..tokens.len() / 2 {
            match tokens[2 * i + 1].parse::<f64>() {
                Ok(score) => values.push((score, tokens[2 * i + 2].clone())),
                Err(_) => return Err(RequestError::InvalidFloatValue),
            };
        }
        Ok(Box::new(ZAddCommand {
            key: tokens[0].clone(),
            values,
        }))
    }
}

impl Command for ZAddCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        // TODO: atomicity
        let sorted_set = match data_store.get_sorted_set_mut(&self.key)? {
            Some(sorted_set) => sorted_set,
            None => {
                let _ = data_store.insert_sorted_set(&self.key);
                data_store.get_sorted_set_mut(&self.key).unwrap().unwrap()
            }
        };
        let mut count = 0;
        for (score, element) in &self.values {
            count += sorted_set.insert(*score, element.clone()) as u64;
        }
        Ok(Box::new(ZAddResult { value: count }))
    }
}

#[cfg(test)]
mod test {
    mod test_set {
        use crate::command::sorted_set::ZAddCommand;
        use crate::command::Command;
        use crate::data_store::DataStore;

        #[test]
        fn should_accept_correct_amount_of_tokens() {
            let err = ZAddCommand::new(vec!["foo".to_string()]).err().unwrap();
            assert_eq!(
                err.to_string(),
                "ERR wrong number of arguments for command".to_string()
            );
            let err = ZAddCommand::new(vec!["foo".to_string(), "bar".to_string()])
                .err()
                .unwrap();
            assert_eq!(
                err.to_string(),
                "ERR wrong number of arguments for command".to_string()
            );
            let err = ZAddCommand::new(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string(),
            ])
            .err()
            .unwrap();
            assert_eq!(err.to_string(), "value is not a valid float".to_string());
            let v = ZAddCommand::new(vec![
                "foo".to_string(),
                "1.0".to_string(),
                "baz".to_string(),
            ])
            .unwrap();
            assert_eq!(v.key, "foo".to_string());
            assert_eq!(v.values, vec![(1.0, "baz".to_string())]);
        }

        #[test]
        fn should_insert_value() {
            let mut ds = DataStore::new();
            let key = "foo".to_string();
            let cmd = ZAddCommand::new(vec![
                key.clone(),
                "2.0".to_string(),
                "v1".to_string(),
                "1.0".to_string(),
                "v2".to_string(),
            ])
            .unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "2".to_string());
            let sorted_set = ds.get_sorted_set_mut(&key).unwrap().unwrap();
            assert_eq!(sorted_set.len(), 2);
            assert_eq!(sorted_set.get("v1").unwrap(), 2.0);
            assert_eq!(sorted_set.get("v2").unwrap(), 1.0);

            // Should return 0 because key "bar" already exists
            let cmd =
                ZAddCommand::new(vec![key.clone(), "3.0".to_string(), "v1".to_string()]).unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "0".to_string());
            let sorted_set = ds.get_sorted_set_mut(&key).unwrap().unwrap();
            assert_eq!(sorted_set.get("v1").unwrap(), 3.0);
        }
    }
}
