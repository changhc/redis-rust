use crate::execution_result::{
    ArrayReply, BulkStringReply, ExecutionResult, NullReply, RespReply, SimpleStringReply,
};

pub struct GetResult {
    pub value: Option<String>,
}

impl ExecutionResult for GetResult {
    fn to_string(&self) -> String {
        match &self.value {
            Some(v) => v.clone(),
            None => "".to_string(),
        }
    }
    fn serialise(&self) -> String {
        match &self.value {
            Some(v) => SimpleStringReply { value: v.clone() }.serialise(),
            None => NullReply {}.serialise(),
        }
    }
}

pub struct MgetResult {
    pub values: Vec<Option<String>>,
}

impl ExecutionResult for MgetResult {
    fn to_string(&self) -> String {
        self.values
            .iter()
            .map(|v| match v {
                Some(v) => v.clone(),
                None => "".to_string(),
            })
            .collect::<Vec<String>>()
            .join(",")
    }
    fn serialise(&self) -> String {
        let mut replies: Vec<Box<dyn RespReply>> = Vec::new();
        for value in &self.values {
            replies.push(match value {
                Some(v) => Box::new(BulkStringReply { value: v.clone() }),
                None => Box::new(NullReply {}),
            });
        }
        ArrayReply { values: replies }.serialise()
    }
}
