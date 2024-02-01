use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::set::SMembersResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct SMembersCommand {
    key: String,
}

impl SMembersCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 1 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(SMembersCommand {
            key: tokens[0].clone(),
        }))
    }
}

impl Command for SMembersCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_set_mut(&self.key) {
            Ok(set_op) => {
                let mut values = vec![];
                if let Some(set) = set_op {
                    for v in set.iter() {
                        values.push(v.to_owned());
                    }
                }
                Ok(Box::new(SMembersResult { values }))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command::set::{SAddCommand, SMembersCommand};
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_one_token() {
        let err = SMembersCommand::new(vec!["foo".to_string(), "bar".to_string()])
            .err()
            .unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
        let v = SMembersCommand::new(vec!["foo".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
    }

    #[test]
    fn should_return_empty_list_if_key_does_not_exist() {
        let cmd = SMembersCommand::new(vec!["foo".to_string()]).unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
    }

    #[test]
    fn should_return_elements() {
        let key = "foo".to_string();
        let mut ds = DataStore::new();
        SAddCommand::new(vec![key.clone(), "v1".to_string(), "v2".to_string()])
            .unwrap()
            .execute(&mut ds)
            .unwrap();
        let cmd = SMembersCommand::new(vec![key.clone()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "v1,v2".to_string());
    }
}
