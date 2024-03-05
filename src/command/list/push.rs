use crate::command::list::OperationDirection;
use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::list::PushResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct PushCommand {
    key: String,
    values: Vec<String>,
    direction: OperationDirection,
}

impl PushCommand {
    pub fn new(
        tokens: Vec<String>,
        direction: OperationDirection,
    ) -> Result<Box<Self>, RequestError> {
        if tokens.len() < 2 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(PushCommand {
            key: tokens[0].clone(),
            values: tokens[1..].to_vec(),
            direction,
        }))
    }
}

impl Command for PushCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        let list = match data_store.get_list_mut(&self.key)? {
            Some(list) => list,
            None => {
                let _ = data_store.insert_list(&self.key);
                data_store.get_list_mut(&self.key).unwrap().unwrap()
            }
        };
        for value in &self.values {
            match &self.direction {
                OperationDirection::Left => list.push_front(value.clone()),
                OperationDirection::Right => list.push_back(value.clone()),
            };
        }
        Ok(Box::new(PushResult { value: list.len() }))
    }
}

#[cfg(test)]
mod test {
    use crate::command::list::OperationDirection;
    use crate::command::list::PushCommand;
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_at_least_two_tokens() {
        let err = PushCommand::new(vec!["foo".to_string()], OperationDirection::Left)
            .err()
            .unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
        let v = PushCommand::new(
            vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
            OperationDirection::Left,
        )
        .unwrap();
        assert_eq!(v.key, "foo".to_string());
        assert_eq!(v.values, vec!["bar".to_string(), "baz".to_string()]);
    }

    #[test]
    fn should_push_item_if_key_does_not_exist_left() {
        let key = "foo".to_string();
        let cmd = PushCommand::new(
            vec![key.clone(), "bar".to_string(), "baz".to_string()],
            OperationDirection::Left,
        )
        .unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "2".to_string());
        let list = ds.get_list_mut(&key).unwrap().unwrap();
        assert_eq!(*list.front().unwrap(), "baz".to_string());
        assert_eq!(*list.back().unwrap(), "bar".to_string());
    }

    #[test]
    fn should_push_item_if_key_does_not_exist_right() {
        let key = "foo".to_string();
        let cmd = PushCommand::new(
            vec![key.clone(), "bar".to_string(), "baz".to_string()],
            OperationDirection::Right,
        )
        .unwrap();
        let mut ds = DataStore::new();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "2".to_string());
        let list = ds.get_list_mut(&key).unwrap().unwrap();
        assert_eq!(*list.front().unwrap(), "bar".to_string());
        assert_eq!(*list.back().unwrap(), "baz".to_string());
    }
}
