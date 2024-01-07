use crate::command::list::OperationDirection;
use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::list::PopResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct PopCommand {
    key: String,
    count: usize,
    direction: OperationDirection,
}

impl PopCommand {
    pub fn new(
        tokens: Vec<String>,
        direction: OperationDirection,
    ) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 1 && tokens.len() != 2 {
            return Err(RequestError::IncorrectArgCount);
        }
        let count = if tokens.len() == 2 {
            match tokens[1].parse::<usize>() {
                Ok(v) => v,
                Err(_) => return Err(RequestError::InvalidNegValue),
            }
        } else {
            1
        };
        Ok(Box::new(PopCommand {
            key: tokens[0].clone(),
            count,
            direction,
        }))
    }
}

impl Command for PopCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_list_mut(&self.key) {
            Ok(list_op) => {
                let values = match list_op {
                    Some(list) => {
                        let mut values = Vec::new();
                        for _ in 0..self.count {
                            let pop_result = match &self.direction {
                                OperationDirection::LEFT => list.pop_front(),
                                OperationDirection::RIGHT => list.pop_back(),
                            };
                            match pop_result {
                                Some(v) => values.push(v),
                                None => break,
                            };
                        }

                        let len = list.len();
                        if len == 0 {
                            data_store.drop_key(&self.key);
                        }
                        values
                    }
                    None => Vec::new(),
                };
                Ok(Box::new(PopResult { values }))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command::list::{OperationDirection, PopCommand, PushCommand};
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_one_or_two_tokens() {
        let v = PopCommand::new(vec!["foo".to_string()], OperationDirection::LEFT).unwrap();
        assert_eq!(v.key, "foo".to_string());
        assert_eq!(v.count, 1);
        let v = PopCommand::new(
            vec!["foo".to_string(), "3".to_string()],
            OperationDirection::LEFT,
        )
        .unwrap();
        assert_eq!(v.key, "foo".to_string());
        assert_eq!(v.count, 3);
    }

    #[test]
    fn should_reject_invalid_count() {
        let expected_msg = "value is out of range, must be positive".to_string();
        let err = PopCommand::new(
            vec!["foo".to_string(), "bar".to_string()],
            OperationDirection::LEFT,
        )
        .err()
        .unwrap();
        assert_eq!(err.to_string(), expected_msg);
        let err = PopCommand::new(
            vec!["foo".to_string(), "-6".to_string()],
            OperationDirection::LEFT,
        )
        .err()
        .unwrap();
        assert_eq!(err.to_string(), expected_msg);
    }

    #[test]
    fn should_pop_item_from_the_front_with_count_1() {
        let key = "foo".to_string();
        let mut ds = DataStore::new();
        let _ = PushCommand::new(
            vec![key.clone(), "bar".to_string(), "baz".to_string()],
            OperationDirection::LEFT,
        )
        .unwrap()
        .execute(&mut ds);
        let result = PopCommand::new(vec![key.clone()], OperationDirection::LEFT)
            .unwrap()
            .execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "baz".to_string());
        let list = ds.get_list_mut(&key).unwrap().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(*list.back().unwrap(), "bar".to_string());
    }

    #[test]
    fn should_pop_item_from_the_front_with_count_greater_than_1_left() {
        let key = "foo".to_string();
        let mut ds = DataStore::new();
        let _ = PushCommand::new(
            vec![
                key.clone(),
                "v0".to_string(),
                "v1".to_string(),
                "v2".to_string(),
                "v3".to_string(),
            ],
            OperationDirection::LEFT,
        )
        .unwrap()
        .execute(&mut ds);
        let result = PopCommand::new(vec![key.clone(), "3".to_string()], OperationDirection::LEFT)
            .unwrap()
            .execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "v3,v2,v1");
        let list = ds.get_list_mut(&key).unwrap().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(*list.back().unwrap(), "v0".to_string());
    }

    #[test]
    fn should_pop_item_from_the_front_with_count_greater_than_1_right() {
        let key = "foo".to_string();
        let mut ds = DataStore::new();
        let _ = PushCommand::new(
            vec![
                key.clone(),
                "v0".to_string(),
                "v1".to_string(),
                "v2".to_string(),
                "v3".to_string(),
            ],
            OperationDirection::LEFT,
        )
        .unwrap()
        .execute(&mut ds);
        let result = PopCommand::new(
            vec![key.clone(), "3".to_string()],
            OperationDirection::RIGHT,
        )
        .unwrap()
        .execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "v0,v1,v2");
        let list = ds.get_list_mut(&key).unwrap().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(*list.back().unwrap(), "v3".to_string());
    }

    #[test]
    fn should_return_nothing_when_key_does_not_exist() {
        let mut ds = DataStore::new();
        let result = PopCommand::new(vec!["foo".to_string()], OperationDirection::LEFT)
            .unwrap()
            .execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
    }

    #[test]
    fn should_remove_key_when_list_is_empty() {
        let key = "foo".to_string();
        let mut ds = DataStore::new();
        let _ = PushCommand::new(
            vec![key.clone(), "bar".to_string()],
            OperationDirection::LEFT,
        )
        .unwrap()
        .execute(&mut ds);
        let result = PopCommand::new(vec![key.clone()], OperationDirection::LEFT)
            .unwrap()
            .execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "bar".to_string());
        let list_op = ds.get_list_mut(&key).unwrap();
        assert!(list_op.is_none());
    }
}
