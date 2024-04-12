use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::sorted_set::ZRankResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct ZRankCommand {
    key: String,
    value: String,
}

impl ZRankCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() != 2 {
            return Err(RequestError::IncorrectArgCount);
        }
        Ok(Box::new(ZRankCommand {
            key: tokens[0].to_owned(),
            value: tokens[1].to_owned(),
        }))
    }
}

impl Command for ZRankCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        let rank = match data_store.get_sorted_set_mut(&self.key)? {
            Some(sorted_set) => sorted_set.get_rank(&self.value),
            None => None,
        };

        Ok(Box::new(ZRankResult { value: rank }))
    }
}

#[cfg(test)]
mod test {
    mod test_set {
        use std::iter::zip;

        use crate::command::sorted_set::{ZAddCommand, ZRankCommand};
        use crate::command::Command;
        use crate::data_store::DataStore;

        #[test]
        fn should_accept_correct_amount_of_tokens() {
            let err = ZRankCommand::new(vec!["foo".to_string()]).err().unwrap();
            assert_eq!(
                err.to_string(),
                "ERR wrong number of arguments for command".to_string()
            );
            let v = ZRankCommand::new(vec!["foo".to_string(), "baz".to_string()]).unwrap();
            assert_eq!(v.key, "foo".to_string());
            assert_eq!(v.value, "baz".to_string());
        }

        #[test]
        fn should_get_rank() {
            let mut ds = DataStore::new();
            let key = "foo".to_string();
            let elements = ["b", "a", "aa", "d", "c"];
            let scores = [0.5, 1.0, 1.0, 1.2, 1.5];
            for (score, item) in zip(scores, elements) {
                ZAddCommand::new(vec![key.clone(), score.to_string(), item.to_string()])
                    .unwrap()
                    .execute(&mut ds)
                    .unwrap();
            }

            // Should return null because key test does not exist
            let cmd = ZRankCommand::new(vec!["test".to_string(), "a".to_owned()]).unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "".to_string());

            // Should return null because element something does not exist
            let cmd = ZRankCommand::new(vec![key.clone(), "something".to_owned()]).unwrap();
            let result = cmd.execute(&mut ds);
            assert_eq!(result.unwrap().to_string(), "".to_string());

            for (i, element) in elements.iter().cloned().enumerate() {
                let cmd = ZRankCommand::new(vec![key.clone(), element.to_owned()]).unwrap();
                let result = cmd.execute(&mut ds);
                assert_eq!(result.unwrap().to_string(), i.to_string());
            }
        }
    }
}
