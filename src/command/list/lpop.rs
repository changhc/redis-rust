use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::list::LpopResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct LpopCommand {
    key: String,
}

impl LpopCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 1 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(LpopCommand {
            key: tokens[0].clone(),
        }))
    }
}

impl Command for LpopCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_list_mut(&self.key) {
            Ok(list_op) => {
                let list_length = match list_op {
                    Some(list) => {
                        list.pop_front();
                        let len = list.len();
                        if len == 0 {
                            data_store.drop_key(&self.key);
                        }
                        len
                    }
                    None => 0,
                };
                Ok(Box::new(LpopResult { value: list_length }))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data_store::DataStore;

    use crate::command::Command;

    use super::super::LpushCommand;
    use super::LpopCommand;

    #[test]
    fn should_accept_at_least_two_tokens() {
        let v = LpopCommand::new(vec!["foo".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
        let err = LpopCommand::new(vec!["foo".to_string(), "bar".to_string()])
            .err()
            .unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
    }

    #[test]
    fn should_pop_item_from_the_front() {
        let key = "foo".to_string();
        let mut ds = DataStore::new();
        let _ = LpushCommand::new(vec![key.clone(), "bar".to_string(), "baz".to_string()])
            .unwrap()
            .execute(&mut ds);
        let result = LpopCommand::new(vec![key.clone()])
            .unwrap()
            .execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "1".to_string());
        let list = ds.get_list_mut(&key).unwrap().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(*list.back().unwrap(), "bar".to_string());
    }

    #[test]
    fn should_remove_key_when_list_is_empty() {
        let key = "foo".to_string();
        let mut ds = DataStore::new();
        let _ = LpushCommand::new(vec![key.clone(), "bar".to_string()])
            .unwrap()
            .execute(&mut ds);
        let result = LpopCommand::new(vec![key.clone()])
            .unwrap()
            .execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "0".to_string());
        let list_op = ds.get_list_mut(&key).unwrap();
        assert!(list_op.is_none());
    }
}
