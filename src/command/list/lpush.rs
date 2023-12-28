use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::list::LpushResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct LpushCommand {
    key: String,
    values: Vec<String>,
}

impl LpushCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() < 2 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(LpushCommand {
            key: tokens[0].clone(),
            values: tokens[1..].to_vec(),
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
                let list = match list_op {
                    Some(list) => list,
                    None => {
                        let _ = data_store.insert_list(&self.key);
                        data_store.get_list_mut(&self.key).unwrap().unwrap()
                    }
                };
                for value in &self.values {
                    list.push_front(value.clone());
                }
                Ok(Box::new(LpushResult { value: list.len() }))
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
    fn should_accept_at_least_two_tokens() {
        let err = LpushCommand::new(vec!["foo".to_string()]).err().unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
        let v = LpushCommand::new(vec![
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
        ])
        .unwrap();
        assert_eq!(v.key, "foo".to_string());
        assert_eq!(v.values, vec!["bar".to_string(), "baz".to_string()]);
    }

    #[test]
    fn should_push_item_if_key_does_not_exist() {
        let key = "foo".to_string();
        let cmd =
            LpushCommand::new(vec![key.clone(), "bar".to_string(), "baz".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "2".to_string());
        let list = ds.get_list_mut(&key).unwrap().unwrap();
        assert_eq!(*list.front().unwrap(), "baz".to_string());
        assert_eq!(*list.back().unwrap(), "bar".to_string());
    }
}
