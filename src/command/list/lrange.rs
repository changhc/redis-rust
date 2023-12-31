use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::list::LrangeResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct LrangeCommand {
    key: String,
    start: i64,
    stop: i64,
}

impl LrangeCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 3 {
            return Err(RequestError::IncorrectArgCount);
        }
        let Ok(start) = tokens[1].parse::<i64>() else {
            return Err(RequestError::InvalidIntValue);
        };
        let Ok(stop) = tokens[2].parse::<i64>() else {
            return Err(RequestError::InvalidIntValue);
        };
        Ok(Box::new(LrangeCommand {
            key: tokens[0].clone(),
            start,
            stop,
        }))
    }
}

impl Command for LrangeCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_list_mut(&self.key) {
            Ok(list_op) => match list_op {
                Some(list) => {
                    let mut values = Vec::new();
                    let start = if self.start >= 0 {
                        self.start
                    } else {
                        list.len() as i64 + self.start
                    };
                    let stop = if self.stop >= 0 {
                        self.stop
                    } else {
                        list.len() as i64 + self.stop
                    };
                    for (idx, item) in list.iter().enumerate() {
                        if idx as i64 >= start && idx as i64 <= stop {
                            values.push(item.clone());
                        }
                    }
                    Ok(Box::new(LrangeResult { values }))
                }
                None => Ok(Box::new(LrangeResult { values: Vec::new() })),
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::LrangeCommand;
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_exactly_3_tokens() {
        let err = LrangeCommand::new(vec!["foo".to_string()]).err().unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
        let v =
            LrangeCommand::new(vec!["foo".to_string(), "1".to_string(), "2".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
        assert_eq!(v.start, 1);
        assert_eq!(v.stop, 2);
    }

    #[test]
    fn should_reject_invalid_start_and_stop() {
        let err = LrangeCommand::new(vec!["foo".to_string(), "bad".to_string(), "2".to_string()])
            .err()
            .unwrap();
        assert_eq!(
            err.to_string(),
            "value is not an integer or out of range".to_string()
        );
    }

    #[test]
    fn should_list_items() {
        let mut ds = DataStore::new();
        let key = "foo".to_string();
        let _ = ds.insert_list(&key);
        let list = ds.get_list_mut(&key).unwrap().unwrap();
        for v in 0..10 {
            list.push_back(v.to_string());
        }
        let cmd = LrangeCommand::new(vec![key.clone(), "1".to_string(), "3".to_string()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "1,2,3".to_string());

        // negative start/stop
        let cmd =
            LrangeCommand::new(vec![key.clone(), "-4".to_string(), "-2".to_string()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "6,7,8".to_string());

        // non-existent key
        let cmd = LrangeCommand::new(vec!["bar".to_string(), "-4".to_string(), "-2".to_string()])
            .unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
    }
}
