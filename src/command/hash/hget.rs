use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::{hash::HGetResult, ExecutionResult};

#[derive(Debug)]
pub struct HGetCommand {
    key: String,
    field: String,
}

impl HGetCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 2 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(HGetCommand {
            key: tokens[0].clone(),
            field: tokens[1].clone(),
        }))
    }
}

impl Command for HGetCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_hash_mut(&self.key) {
            Ok(hash_op) => Ok(Box::new(HGetResult {
                value: match hash_op {
                    Some(hash) => hash.get(&self.field).cloned(),
                    None => None,
                },
            })),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command::hash::{HGetCommand, HSetCommand};
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_exactly_two_tokens() {
        let v = HGetCommand::new(vec!["foo".to_string(), "bar".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
        assert_eq!(v.field, "bar".to_string());
        let err = HGetCommand::new(vec!["foo".to_string()]).err().unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
    }

    #[test]
    fn should_get_value_if_key_and_field_exist() {
        let mut ds = DataStore::new();
        let key = "foo".to_string();
        let field = "k1".to_string();
        HSetCommand::new(vec![key.clone(), field.clone(), "v1".to_string()])
            .unwrap()
            .execute(&mut ds)
            .unwrap();
        let cmd = HGetCommand::new(vec![key.clone(), field.clone()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "v1".to_string());
    }

    #[test]
    fn should_return_null_if_field_does_not_exist() {
        let mut ds = DataStore::new();
        let key = "foo".to_string();
        let field = "k1".to_string();
        HSetCommand::new(vec![key.clone(), field.clone(), "v1".to_string()])
            .unwrap()
            .execute(&mut ds)
            .unwrap();
        let cmd = HGetCommand::new(vec![key.clone(), "k2".to_string()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
    }

    #[test]
    fn should_return_null_if_key_does_not_exist() {
        let cmd = HGetCommand::new(vec!["foo".to_string(), "bar".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
    }
}
