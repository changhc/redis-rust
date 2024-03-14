use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::{IncrCommandError, RequestError};
use crate::execution_result::{string::IntOpResult, ExecutionResult};

pub enum NumOperator {
    Incr,
    Decr,
}

fn _execute(
    key: &String,
    value: i64,
    data_store: &mut DataStore,
) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
    let default = "0".to_string();
    let curr_value = match data_store.get_string(key)? {
        Some(v) => v,
        None => &default,
    };
    match curr_value.parse::<i64>() {
        Ok(v) => match v.checked_add(value) {
            Some(updated) => {
                let _ = data_store.set_string(key, &updated.to_string());
                Ok(Box::new(IntOpResult { value: updated }))
            }
            None => Err(Box::new(IncrCommandError::ResultOverflow)),
        },
        Err(_) => Err(Box::new(IncrCommandError::InvalidValue)),
    }
}

#[derive(Debug)]
pub struct IncrCommand {
    key: String,
    value: i64,
}

impl IncrCommand {
    pub fn new(tokens: Vec<String>, op: NumOperator) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 1 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(IncrCommand {
            key: tokens[0].clone(),
            value: match op {
                NumOperator::Decr => -1,
                NumOperator::Incr => 1,
            },
        }))
    }
}

impl Command for IncrCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        _execute(&self.key, self.value, data_store)
    }
}

#[derive(Debug)]
pub struct IncrbyCommand {
    key: String,
    value: i64,
}

impl IncrbyCommand {
    pub fn new(tokens: Vec<String>, op: NumOperator) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 2 {
            return Err(RequestError::IncorrectArgCount);
        }

        match tokens[1].parse::<i64>() {
            Ok(increment) => {
                let value = match op {
                    NumOperator::Decr => match increment.checked_neg() {
                        Some(v) => v,
                        None => return Err(RequestError::InvalidIntValue),
                    },
                    NumOperator::Incr => increment,
                };
                Ok(Box::new(IncrbyCommand {
                    key: tokens[0].clone(),
                    value,
                }))
            }
            Err(_) => Err(RequestError::InvalidIntValue),
        }
    }
}

impl Command for IncrbyCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        _execute(&self.key, self.value, data_store)
    }
}

#[cfg(test)]
mod test {
    mod test_incr {
        use crate::command::string::NumOperator;
        use crate::command::Command;
        use crate::data_store::DataStore;

        use super::super::IncrCommand;
        use crate::error::IncrCommandError;

        #[test]
        fn should_accept_exactly_one_token() {
            let err = IncrCommand::new(
                vec!["foo".to_string(), "bar".to_string()],
                NumOperator::Incr,
            )
            .err()
            .unwrap();
            assert_eq!(
                err.to_string(),
                "ERR wrong number of arguments for command".to_string()
            );
            let v = IncrCommand::new(vec!["foo".to_string()], NumOperator::Incr).unwrap();
            assert_eq!(v.key, "foo".to_string());
        }

        #[test]
        fn should_insert_value_when_key_is_not_set() {
            let key = "foo".to_string();
            let cmd = IncrCommand::new(vec![key.clone()], NumOperator::Incr).unwrap();
            let mut ds = DataStore::new();
            assert!(ds.get_string(&key).unwrap().is_none());
            cmd.execute(&mut ds).unwrap();
            assert_eq!(ds.get_string(&key).unwrap().unwrap(), &"1".to_string());
        }

        #[test]
        fn should_insert_value_when_key_is_not_set_decr() {
            let key = "foo".to_string();
            let cmd = IncrCommand::new(vec![key.clone()], NumOperator::Decr).unwrap();
            let mut ds = DataStore::new();
            assert!(ds.get_string(&key).unwrap().is_none());
            cmd.execute(&mut ds).unwrap();
            assert_eq!(ds.get_string(&key).unwrap().unwrap(), &"-1".to_string());
        }

        #[test]
        fn should_throw_error_when_value_is_not_int() {
            let key = "foo".to_string();
            let cmd = IncrCommand::new(vec![key.clone()], NumOperator::Incr).unwrap();
            let mut ds = DataStore::new();
            ds.set_string_overwrite(&key, "bar");
            assert!(cmd.execute(&mut ds).is_err());
        }

        #[test]
        fn should_throw_error_when_value_overflows_incr() {
            let key = "foo".to_string();
            let cmd = IncrCommand::new(vec![key.clone()], NumOperator::Incr).unwrap();
            let mut ds = DataStore::new();
            ds.set_string_overwrite(&key, &i64::MAX.to_string());
            let err = cmd.execute(&mut ds).err().unwrap();
            assert_eq!(
                err.to_string(),
                IncrCommandError::ResultOverflow.to_string()
            )
        }

        #[test]
        fn should_throw_error_when_value_overflows_decr() {
            let key = "foo".to_string();
            let cmd = IncrCommand::new(vec![key.clone()], NumOperator::Decr).unwrap();
            let mut ds = DataStore::new();
            ds.set_string_overwrite(&key, &i64::MIN.to_string());
            let err = cmd.execute(&mut ds).err().unwrap();
            assert_eq!(
                err.to_string(),
                IncrCommandError::ResultOverflow.to_string()
            )
        }
    }
    mod test_incrby {
        use super::super::IncrbyCommand;
        use crate::command::string::NumOperator;
        use crate::command::Command;
        use crate::data_store::DataStore;
        use crate::error::{IncrCommandError, RequestError};

        #[test]
        fn should_accept_exactly_two_tokens() {
            let err = IncrbyCommand::new(vec!["foo".to_string()], NumOperator::Incr)
                .err()
                .unwrap();
            assert_eq!(
                err.to_string(),
                "ERR wrong number of arguments for command".to_string()
            );

            let v = IncrbyCommand::new(vec!["foo".to_string(), "5".to_string()], NumOperator::Incr)
                .unwrap();
            assert_eq!(v.key, "foo".to_string());
            assert_eq!(v.value, 5);
        }

        #[test]
        fn should_reject_non_int_increment() {
            let err = IncrbyCommand::new(
                vec!["foo".to_string(), "bar".to_string()],
                NumOperator::Incr,
            )
            .err()
            .unwrap();
            assert_eq!(err.to_string(), RequestError::InvalidIntValue.to_string());
        }

        #[test]
        fn should_throw_error_when_value_overflows_incr() {
            let key = "foo".to_string();
            let cmd =
                IncrbyCommand::new(vec![key.clone(), "5".to_string()], NumOperator::Incr).unwrap();
            let mut ds = DataStore::new();
            ds.set_string_overwrite(&key, &i64::MAX.to_string());
            let err = cmd.execute(&mut ds).err().unwrap();
            assert_eq!(
                err.to_string(),
                IncrCommandError::ResultOverflow.to_string()
            )
        }

        #[test]
        fn should_throw_error_when_value_overflows_decr() {
            let key = "foo".to_string();
            let cmd =
                IncrbyCommand::new(vec![key.clone(), "5".to_string()], NumOperator::Decr).unwrap();
            let mut ds = DataStore::new();
            ds.set_string_overwrite(&key, &i64::MIN.to_string());
            let err = cmd.execute(&mut ds).err().unwrap();
            assert_eq!(
                err.to_string(),
                IncrCommandError::ResultOverflow.to_string()
            )
        }
    }
}
