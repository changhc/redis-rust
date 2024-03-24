use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::list::LRangeResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct ZRangeCommand {
    key: String,
    start: i64,
    stop: i64,
}

impl ZRangeCommand {
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
        Ok(Box::new(ZRangeCommand {
            key: tokens[0].clone(),
            start,
            stop,
        }))
    }
}

impl Command for ZRangeCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        match data_store.get_sorted_set_mut(&self.key)? {
            Some(sorted_set) => {
                let size = sorted_set.len() as u64;
                let start = match self.start >= 0 {
                    true => self.start as u64,
                    false => (size as i64 + self.start) as u64,
                };
                let stop = match self.stop >= 0 {
                    true => self.stop as u64,
                    false => (size as i64 + self.stop) as u64,
                };
                Ok(Box::new(LRangeResult {
                    values: sorted_set.get_values_by_rank(start, stop),
                }))
            }
            None => Ok(Box::new(LRangeResult { values: Vec::new() })),
        }
    }
}

#[cfg(test)]
mod test {
    use super::ZRangeCommand;
    use crate::command::sorted_set::ZAddCommand;
    use crate::command::Command;
    use crate::data_store::DataStore;

    #[test]
    fn should_accept_exactly_3_tokens() {
        let err = ZRangeCommand::new(vec!["foo".to_string()]).err().unwrap();
        assert_eq!(
            err.to_string(),
            "ERR wrong number of arguments for command".to_string()
        );
        let v =
            ZRangeCommand::new(vec!["foo".to_string(), "1".to_string(), "2".to_string()]).unwrap();
        assert_eq!(v.key, "foo".to_string());
        assert_eq!(v.start, 1);
        assert_eq!(v.stop, 2);
    }

    #[test]
    fn should_reject_invalid_start_and_stop() {
        let err = ZRangeCommand::new(vec!["foo".to_string(), "bad".to_string(), "2".to_string()])
            .err()
            .unwrap();
        assert_eq!(
            err.to_string(),
            "ERR value is not an integer or out of range".to_string()
        );
    }

    #[test]
    fn should_list_items() {
        let mut ds = DataStore::new();
        let key = "foo".to_string();
        let input = [(1.0, "a"), (0.5, "b"), (1.0, "aa"), (1.5, "c"), (1.2, "d")];
        for (score, item) in input {
            ZAddCommand::new(vec![key.clone(), score.to_string(), item.to_string()])
                .unwrap()
                .execute(&mut ds)
                .unwrap();
        }
        let cmd = ZRangeCommand::new(vec![key.clone(), "0".to_string(), "1".to_string()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "b,a".to_string());

        // negative start/stop
        let cmd =
            ZRangeCommand::new(vec![key.clone(), "-4".to_string(), "-2".to_string()]).unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "a,aa,d".to_string());

        // non-existent key
        let cmd = ZRangeCommand::new(vec!["bar".to_string(), "-4".to_string(), "-2".to_string()])
            .unwrap();
        let result = cmd.execute(&mut ds);
        assert_eq!(result.unwrap().to_string(), "".to_string());
    }
}
