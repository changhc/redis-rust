use crate::command::Command;
use crate::data_store::DataStore;
use crate::error::RequestError;
use crate::execution_result::sorted_set::ZAddResult;
use crate::execution_result::ExecutionResult;

#[derive(Debug)]
pub struct XAddCommand {
    key: String,
    values: Vec<(f64, String)>,
}

impl XAddCommand {
    pub fn new(tokens: Vec<String>) -> Result<Box<Self>, RequestError> {
        if tokens.len() % 2 != 1 || tokens.len() < 3 {
            return Err(RequestError::IncorrectArgCount);
        }
        let mut values = Vec::new();
        for i in 0..tokens.len() / 2 {
            match tokens[2 * i + 1].parse::<f64>() {
                Ok(score) => values.push((score, tokens[2 * i + 2].clone())),
                Err(_) => return Err(RequestError::InvalidFloatValue),
            };
        }
        Ok(Box::new(XAddCommand {
            key: tokens[0].clone(),
            values,
        }))
    }
}

impl Command for XAddCommand {
    fn execute(
        &self,
        data_store: &mut DataStore,
    ) -> Result<Box<dyn ExecutionResult>, Box<dyn std::error::Error>> {
        // TODO: atomicity
        let sorted_set = match data_store.get_sorted_set_mut(&self.key)? {
            Some(sorted_set) => sorted_set,
            None => {
                let _ = data_store.insert_sorted_set(&self.key);
                data_store.get_sorted_set_mut(&self.key).unwrap().unwrap()
            }
        };
        let mut count = 0;
        for (score, element) in &self.values {
            count += sorted_set.insert(*score, element.clone()) as u64;
        }
        Ok(Box::new(ZAddResult { value: count }))
    }
}
