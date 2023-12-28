use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::list::LpushResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct LpushCommand {
    key: String,
    value: String,
}

impl LpushCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 2 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(LpushCommand {
            key: tokens[0].clone(),
            value: tokens[1].clone(),
        }))
    }
}

impl Command for LpushCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_list_mut(&self.key) {
            Ok(list_op) => {
                match list_op {
                    Some(list) => list.push_back(self.value.clone()),
                    None => {
                        let _ = data_store.insert_list(&self.key, &self.value);
                    }
                };
                Ok(Box::new(LpushResult {}))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data_store::DataStore;

    use crate::command::Command;

    use super::LpushCommand;

    #[test]
    fn should_accept_exactly_one_token() {
        match LpushCommand::new(vec!["foo".to_string()]) {
            Ok(v) => {
                assert_eq!(v.key, "foo".to_string());
            }
            Err(_) => panic!("should be ok"),
        }
        match LpushCommand::new(vec!["foo".to_string(), "bar".to_string()]) {
            Ok(_) => panic!("should not be ok"),
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    "ERR wrong number of arguments for command".to_string()
                );
            }
        }
    }

    #[test]
    fn should_get_value_if_key_exists() {
        let cmd = LpushCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = DataStore::new();
        ds.set_string_overwrite(&"foo".to_string(), &"bar".to_string());
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "bar".to_string());
    }

    #[test]
    fn should_return_null_if_key_does_not_exist() {
        let cmd = LpushCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
    }
}
