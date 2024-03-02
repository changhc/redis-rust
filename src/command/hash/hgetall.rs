use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::{hash::HGetAllResult, ExecutionResult};

#[derive(Debug)]
pub struct HGetAllCommand {
    key: String,
}

impl HGetAllCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 1 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(HGetAllCommand {
            key: tokens[0].clone(),
        }))
    }
}

impl Command for HGetAllCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_hash_mut(&self.key) {
            Ok(hash_op) => Ok(Box::new(HGetAllResult {
                values: match hash_op {
                    Some(hash) => hash
                        .iter()
                        .flat_map(|(k, v)| vec![k.clone(), v.clone()])
                        .collect(),
                    None => vec![],
                },
            })),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command::hash::{HGetAllCommand, HSetCommand};
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_exactly_two_tokens() {
        let v = HGetAllCommand::new(vec!["foo".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
        let err = HGetAllCommand::new(vec!["foo".to_string(), "bar".to_string()])
            .err()
            .unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
    }

    #[test]
    fn should_return_key_value() {
        let mut ds = DataStore::new();
        let key = "foo".to_string();
        HSetCommand::new(vec![
            key.clone(),
            "k1".to_string(),
            "v1".to_string(),
            "k2".to_string(),
            "v2".to_string(),
        ])
        .unwrap()
        .execute(&mut ds)
        .unwrap();
        let cmd = HGetAllCommand::new(vec![key.clone()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "k1,v1,k2,v2".to_string());
    }

    #[test]
    fn should_return_empty_vec_if_key_does_not_exist() {
        let cmd = HGetAllCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
    }
}
