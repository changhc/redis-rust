use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::set::SIsmemberResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct SIsmemberCommand {
    key: String,
    value: String,
}

impl SIsmemberCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 2 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(SIsmemberCommand {
            key: tokens[0].clone(),
            value: tokens[1].clone(),
        }))
    }
}

impl Command for SIsmemberCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_set_mut(&self.key) {
            Ok(set_op) => {
                let count = match set_op {
                    Some(set) => match set.contains(&self.value) {
                        true => 1,
                        _ => 0,
                    },
                    None => 0,
                };
                Ok(Box::new(SIsmemberResult { value: count }))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command::set::{SAddCommand, SIsmemberCommand};
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_at_two_tokens() {
        let err = SIsmemberCommand::new(vec!["foo".to_string()])
            .err()
            .unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
        let v = SIsmemberCommand::new(vec!["foo".to_string(), "bar".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
        assert_eq!(v.value, "bar".to_string());
    }

    #[test]
    fn should_throw_error_for_non_set_keys() {
        let key = "foo".to_string();
        let cmd = SIsmemberCommand::new(vec![key.clone(), "bar".to_string()]).unwrap();
        let mut ds = DataStore::new();
        ds.set_string(&key, "something").unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(
            result.err().unwrap().to_string(),
            "WRONGTYPE Operation against a key holding the wrong kind of value".to_string()
        );
    }

    #[test]
    fn should_return_0_if_key_does_not_exist() {
        let key = "foo".to_string();
        let cmd = SIsmemberCommand::new(vec![key.clone(), "bar".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "0".to_string());
    }

    #[test]
    fn should_return_exists_or_not() {
        let key = "foo".to_string();
        let mut ds = DataStore::new();
        // insert set
        SAddCommand::new(vec![key.clone(), "v1".to_string()])
            .unwrap()
            .execute(&mut ds)
            .unwrap();

        let cmd = SIsmemberCommand::new(vec![key.clone(), "v1".to_string()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "1".to_string());
        let cmd = SIsmemberCommand::new(vec![key.clone(), "v2".to_string()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "0".to_string());
    }
}
