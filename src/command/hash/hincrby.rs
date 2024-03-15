use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::{HIncrByCommandError, IncrCommandError, RequestError};
use crate::execution_result::hash::HIncrByResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct HIncrByCommand {
    key: String,
    field: String,
    amount: i64,
}

impl HIncrByCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 3 {
            return Err(RequestError::IncorrectArgCount);
        }
        match tokens[2].parse::<i64>() {
            Ok(amount) => Ok(Box::new(HIncrByCommand {
                key: tokens[0].clone(),
                field: tokens[1].clone(),
                amount,
            })),
            Err(_) => Err(RequestError::InvalidIntValue),
        }
    }
}

impl Command for HIncrByCommand {
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
        let result = match hash.get(&self.field) {
            Some(v) => match v.parse::<i64>() {
                Ok(current_value) => match current_value.checked_add(self.amount) {
                    Some(result) => {
                        hash.insert(self.field.clone(), result.to_string());
                        result
                    }
                    None => return Err(Box::new(IncrCommandError::ResultOverflow)),
                },
                Err(_) => return Err(Box::new(HIncrByCommandError::InvalidHashValue)),
            },
            None => {
                hash.insert(self.field.clone(), self.amount.to_string());
                self.amount
            }
        };
        Ok(Box::new(HIncrByResult { value: result }))
    }
}

#[cfg(test)]
mod test {
    mod test_set {
        use crate::command::hash::{HIncrByCommand, HSetCommand};
        use crate::command::Command;
        use crate::data_store::DataStore;
        use crate::error::{HIncrByCommandError, IncrCommandError, RequestError};

        #[test]
        fn should_accept_3_tokens() {
            let err = HIncrByCommand::new(vec!["foo".to_string(), "bar".to_string()])
                .err()
                .unwrap();
            assert_eq!(
                err.to_string(),
                "ERR wrong number of arguments for command".to_string()
            );
            let err = HIncrByCommand::new(vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string(),
            ])
            .err()
            .unwrap();
            assert_eq!(err.to_string(), RequestError::InvalidIntValue.to_string());
            let v =
                HIncrByCommand::new(vec!["foo".to_string(), "bar".to_string(), "1".to_string()])
                    .unwrap();
            assert_eq!(v.key, "foo".to_string());
            assert_eq!(v.field, "bar".to_string());
            assert_eq!(v.amount, 1);
        }

        #[test]
        fn should_set_value_when_key_does_not_exist() {
            let mut ds = DataStore::new();
            let key = "foo".to_string();
            let cmd =
                HIncrByCommand::new(vec![key.clone(), "k1".to_string(), "2".to_string()]).unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "2".to_string());
            let hash = ds.get_hash_mut(&key).unwrap().unwrap();
            assert_eq!(hash.len(), 1);
            assert_eq!(hash.get(&"k1".to_string()).unwrap(), &"2".to_string());
        }

        #[test]
        fn should_set_value_when_field_does_not_exist() {
            let mut ds = DataStore::new();
            let key = "foo".to_string();
            HSetCommand::new(vec![key.clone(), "k0".to_string(), "v0".to_string()])
                .unwrap()
                .execute(&mut ds)
                .unwrap();
            let hash = ds.get_hash_mut(&key).unwrap().unwrap();
            assert_eq!(hash.len(), 1);

            let cmd =
                HIncrByCommand::new(vec![key.clone(), "k1".to_string(), "2".to_string()]).unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "2".to_string());
            let hash = ds.get_hash_mut(&key).unwrap().unwrap();
            assert_eq!(hash.len(), 2);
            assert_eq!(hash.get(&"k1".to_string()).unwrap(), &"2".to_string());
        }

        #[test]
        fn should_increase_value_when_a_valid_field_exists() {
            let mut ds = DataStore::new();
            let key = "foo".to_string();
            let cmd =
                HIncrByCommand::new(vec![key.clone(), "k1".to_string(), "2".to_string()]).unwrap();
            cmd.execute(&mut ds).unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "4".to_string());
            let hash = ds.get_hash_mut(&key).unwrap().unwrap();
            assert_eq!(hash.get(&"k1".to_string()).unwrap(), &"4".to_string());
        }

        #[test]
        fn should_throw_error_for_nonnumerical_value() {
            let mut ds = DataStore::new();
            let key = "foo".to_string();
            HSetCommand::new(vec![key.clone(), "k1".to_string(), "v1".to_string()])
                .unwrap()
                .execute(&mut ds)
                .unwrap();
            let cmd =
                HIncrByCommand::new(vec![key.clone(), "k1".to_string(), "2".to_string()]).unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(
                result.err().unwrap().to_string(),
                HIncrByCommandError::InvalidHashValue.to_string()
            );
        }

        #[test]
        fn should_throw_error_when_overflow() {
            let mut ds = DataStore::new();
            let key = "foo".to_string();
            HSetCommand::new(vec![key.clone(), "k1".to_string(), i64::MAX.to_string()])
                .unwrap()
                .execute(&mut ds)
                .unwrap();

            let result = HIncrByCommand::new(vec![key.clone(), "k1".to_string(), "1".to_string()])
                .unwrap()
                .execute(&mut ds);
            assert_eq!(
                result.err().unwrap().to_string(),
                IncrCommandError::ResultOverflow.to_string()
            );

            HSetCommand::new(vec![key.clone(), "k1".to_string(), i64::MIN.to_string()])
                .unwrap()
                .execute(&mut ds)
                .unwrap();
            let result = HIncrByCommand::new(vec![key.clone(), "k1".to_string(), "-1".to_string()])
                .unwrap()
                .execute(&mut ds);
            assert_eq!(
                result.err().unwrap().to_string(),
                IncrCommandError::ResultOverflow.to_string()
            );
        }
    }
}
