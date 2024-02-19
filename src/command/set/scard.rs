use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::set::SCardResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct SCardCommand {
    key: String,
}

impl SCardCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 1 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(SCardCommand {
            key: tokens[0].clone(),
        }))
    }
}

impl Command for SCardCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_set_mut(&self.key) {
            Ok(set_op) => {
                let count = match set_op {
                    Some(set) => set.len(),
                    None => 0,
                };
                Ok(Box::new(SCardResult { value: count }))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command::set::{SAddCommand, SCardCommand};
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_at_least_two_tokens() {
        let err = SCardCommand::new(vec!["foo".to_string(), "bar".to_string()])
            .err()
            .unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
        let v = SCardCommand::new(vec!["foo".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
    }

    #[test]
    fn should_return_0_if_key_does_not_exist() {
        let cmd = SCardCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "0".to_string());
    }

    #[test]
    fn should_return_card() {
        let mut ds = DataStore::new();
        let key = "foo".to_string();
        SAddCommand::new(vec![key.clone(), "bar".to_string()])
            .unwrap()
            .execute(&mut ds)
            .unwrap();
        let cmd = SCardCommand::new(vec![key.clone()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "1".to_string());
    }
}
