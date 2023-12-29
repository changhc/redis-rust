use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::list::LpopResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct LpopCommand {
    key: String,
    count: usize,
}

impl LpopCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
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
        Ok(Box::new(LpopCommand {
            key: tokens[0].clone(),
            count: count,
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
                let values = match list_op {
                    Some(list) => {
                        let mut values = Vec::new();
                        for _ in 0..self.count {
                            match list.pop_front() {
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
                Ok(Box::new(LpopResult { values: values }))
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
    fn should_accept_one_or_two_tokens() {
        let v = LpopCommand::new(vec!["foo".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
        assert_eq!(v.count, 1);
        let v = LpopCommand::new(vec!["foo".to_string(), "3".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
        assert_eq!(v.count, 3);
    }

    #[test]
    fn should_reject_invalid_count() {
        let expected_msg = "value is out of range, must be positive".to_string();
        let err = LpopCommand::new(vec!["foo".to_string(), "bar".to_string()])
            .err()
            .unwrap();
        assert_eq!(err.to_string(), expected_msg);
        let err = LpopCommand::new(vec!["foo".to_string(), "-6".to_string()])
            .err()
            .unwrap();
        assert_eq!(err.to_string(), expected_msg);
    }

    #[test]
    fn should_pop_item_from_the_front_with_count_1() {
        let key = "foo".to_string();
        let mut ds = DataStore::new();
        let _ = LpushCommand::new(vec![key.clone(), "bar".to_string(), "baz".to_string()])
            .unwrap()
            .execute(&mut ds);
        let result = LpopCommand::new(vec![key.clone()])
            .unwrap()
            .execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "baz".to_string());
        let list = ds.get_list_mut(&key).unwrap().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(*list.back().unwrap(), "bar".to_string());
    }

    #[test]
    fn should_pop_item_from_the_front_with_count_greater_than_1() {
        let key = "foo".to_string();
        let mut ds = DataStore::new();
        let _ = LpushCommand::new(vec![
            key.clone(),
            "v0".to_string(),
            "v1".to_string(),
            "v2".to_string(),
            "v3".to_string(),
        ])
        .unwrap()
        .execute(&mut ds);
        let result = LpopCommand::new(vec![key.clone(), "3".to_string()])
            .unwrap()
            .execute(&mut ds);
        assert_eq!(
            result
                .unwrap()
                .to_string()
                .split("\r\n")
                .collect::<Vec<_>>()[1..],
            vec!["v3".to_string(), "v2".to_string(), "v1".to_string()]
        );
        let list = ds.get_list_mut(&key).unwrap().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(*list.back().unwrap(), "v0".to_string());
    }

    #[test]
    fn should_return_nothing_when_key_does_not_exist() {
        let mut ds = DataStore::new();
        let result = LpopCommand::new(vec!["foo".to_string()])
            .unwrap()
            .execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
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
        assert_eq!(result.unwrap().to_string(), "bar".to_string());
        let list_op = ds.get_list_mut(&key).unwrap();
        assert!(list_op.is_none());
    }
}
