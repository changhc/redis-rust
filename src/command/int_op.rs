use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::{IncrCommandError, RequestError};
use crate::execution_result::{ExecutionResult, IntOpResult};

pub enum NumOperator {
    INCR,
    DECR,
}

fn _execute(
    key: &String,
    value: i64,
    data_store: &mut DataStore,
) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
    let default = "0".to_string();
    let curr_value = match data_store.get_string(key) {
        Ok(op) => match op {
            Some(v) => v,
            None => &default,
        },
        Err(e) => return Err(e),
    };
    match curr_value.parse::<i64>() {
        Ok(v) => match v.checked_add(value) {
            Some(updated) => {
                let _ = data_store.set_string(key, &updated.to_string());
                Ok(Box::new(IntOpResult { value: updated }))
            }
            None => Err(Box::new(IncrCommandError::InvalidValue)),
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
                NumOperator::DECR => -1,
                NumOperator::INCR => 1,
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
                    NumOperator::DECR => match increment.checked_neg() {
                        Some(v) => v,
                        None => return Err(RequestError::InvalidIntValue),
                    },
                    NumOperator::INCR => increment,
                };
                Ok(Box::new(IncrbyCommand {
                    key: tokens[0].clone(),
                    value: value,
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
        use crate::command::{int_op::NumOperator, Command};
        use crate::data_store::DataStore;

        use super::super::IncrCommand;
        use crate::error::IncrCommandError;

        #[test]
        fn should_accept_exactly_one_token() {
            match IncrCommand::new(
                vec!["foo".to_string(), "bar".to_string()],
                NumOperator::INCR,
            ) {
                Ok(_) => panic!("should not be ok"),
                Err(e) => {
                    assert_eq!(
                        e.to_string(),
                        "ERR wrong number of arguments for command".to_string()
                    );
                }
            }
            match IncrCommand::new(vec!["foo".to_string()], NumOperator::INCR) {
                Ok(v) => {
                    assert_eq!(v.key, "foo".to_string());
                }
                Err(_) => panic!("should be ok"),
            }
        }

        #[test]
        fn should_insert_value_when_key_is_not_set() {
            let key = "foo".to_string();
            let cmd = IncrCommand::new(vec![key.clone()], NumOperator::INCR).unwrap();
            let mut ds = DataStore::new();
            assert!(ds.get_string(&key).unwrap().is_none());
            cmd.execute(&mut ds).unwrap();
            assert_eq!(ds.get_string(&key).unwrap().unwrap(), &"1".to_string());
        }

        #[test]
        fn should_insert_value_when_key_is_not_set_decr() {
            let key = "foo".to_string();
            let cmd = IncrCommand::new(vec![key.clone()], NumOperator::DECR).unwrap();
            let mut ds = DataStore::new();
            assert!(ds.get_string(&key).unwrap().is_none());
            cmd.execute(&mut ds).unwrap();
            assert_eq!(ds.get_string(&key).unwrap().unwrap(), &"-1".to_string());
        }

        #[test]
        fn should_throw_error_when_value_is_not_int() {
            let key = "foo".to_string();
            let cmd = IncrCommand::new(vec![key.clone()], NumOperator::INCR).unwrap();
            let mut ds = DataStore::new();
            ds.set_string_overwrite(&key, &"bar".to_string());
            assert!(cmd.execute(&mut ds).is_err());
        }

        #[test]
        fn should_throw_error_when_value_overflows_incr() {
            let key = "foo".to_string();
            let cmd = IncrCommand::new(vec![key.clone()], NumOperator::INCR).unwrap();
            let mut ds = DataStore::new();
            ds.set_string_overwrite(&key, &i64::MAX.to_string());
            match cmd.execute(&mut ds) {
                Ok(_) => panic!("should not be ok"),
                Err(e) => assert_eq!(e.to_string(), IncrCommandError::InvalidValue.to_string()),
            }
        }

        #[test]
        fn should_throw_error_when_value_overflows_decr() {
            let key = "foo".to_string();
            let cmd = IncrCommand::new(vec![key.clone()], NumOperator::DECR).unwrap();
            let mut ds = DataStore::new();
            ds.set_string_overwrite(&key, &i64::MIN.to_string());
            match cmd.execute(&mut ds) {
                Ok(_) => panic!("should not be ok"),
                Err(e) => assert_eq!(e.to_string(), IncrCommandError::InvalidValue.to_string()),
            }
        }
    }
    mod test_incrby {
        use super::super::IncrbyCommand;
        use crate::command::Command;
        use crate::data_store::DataStore;
        use crate::error::RequestError;
        use crate::{command::int_op::NumOperator, error::IncrCommandError};

        #[test]
        fn should_accept_exactly_two_tokens() {
            match IncrbyCommand::new(vec!["foo".to_string()], NumOperator::INCR) {
                Ok(_) => panic!("should not be ok"),
                Err(e) => {
                    assert_eq!(
                        e.to_string(),
                        "ERR wrong number of arguments for command".to_string()
                    );
                }
            }
            match IncrbyCommand::new(vec!["foo".to_string(), "5".to_string()], NumOperator::INCR) {
                Ok(v) => {
                    assert_eq!(v.key, "foo".to_string());
                }
                Err(_) => panic!("should be ok"),
            }
        }

        #[test]
        fn should_reject_non_int_increment() {
            match IncrbyCommand::new(
                vec!["foo".to_string(), "bar".to_string()],
                NumOperator::INCR,
            ) {
                Ok(_) => panic!("should not be ok"),
                Err(e) => assert_eq!(e.to_string(), RequestError::InvalidIntValue.to_string()),
            }
        }

        #[test]
        fn should_throw_error_when_value_overflows_incr() {
            let key = "foo".to_string();
            let cmd =
                IncrbyCommand::new(vec![key.clone(), "5".to_string()], NumOperator::INCR).unwrap();
            let mut ds = DataStore::new();
            ds.set_string_overwrite(&key, &i64::MAX.to_string());
            match cmd.execute(&mut ds) {
                Ok(_) => panic!("should not be ok"),
                Err(e) => assert_eq!(e.to_string(), IncrCommandError::InvalidValue.to_string()),
            }
        }

        #[test]
        fn should_throw_error_when_value_overflows_decr() {
            let key = "foo".to_string();
            let cmd =
                IncrbyCommand::new(vec![key.clone(), "5".to_string()], NumOperator::DECR).unwrap();
            let mut ds = DataStore::new();
            ds.set_string_overwrite(&key, &i64::MIN.to_string());
            match cmd.execute(&mut ds) {
                Ok(_) => panic!("should not be ok"),
                Err(e) => assert_eq!(e.to_string(), IncrCommandError::InvalidValue.to_string()),
            }
        }
    }
}
