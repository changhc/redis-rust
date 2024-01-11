use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::list::LLenResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct LLenCommand {
    key: String,
}

impl LLenCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 1 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(LLenCommand {
            key: tokens[0].clone(),
        }))
    }
}

impl Command for LLenCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_list_mut(&self.key) {
            Ok(list_op) => match list_op {
                Some(list) => Ok(Box::new(LLenResult { value: list.len() })),
                None => Ok(Box::new(LLenResult { value: 0 })),
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::LLenCommand;
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_exactly_one_token() {
        let err = LLenCommand::new(vec!["foo".to_string(), "bar".to_string()])
            .err()
            .unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
        let v = LLenCommand::new(vec!["foo".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
    }

    #[test]
    fn should_return_list_length() {
        let mut ds = DataStore::new();
        let key = "foo".to_string();
        let _ = ds.insert_list(&key);
        let list = ds.get_list_mut(&key).unwrap().unwrap();
        for v in 0..10 {
            list.push_back(v.to_string());
        }
        let cmd = LLenCommand::new(vec![key.clone()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "10".to_string());

        // non-existent key
        let cmd = LLenCommand::new(vec!["bar".to_string()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "0".to_string());
    }
}
