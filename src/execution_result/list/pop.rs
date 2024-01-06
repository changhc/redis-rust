use crate::execution_result::{
    ArrayReply, BulkStringReply, ExecutionResult, NullReply, RespReply, SimpleStringReply,
};

pub struct PopResult {
    pub values: Vec<String>,
}

impl ExecutionResult for PopResult {
    fn to_string(&self) -> String {
        self.values.join(",")
    }
    fn serialise(&self) -> String {
        let v: Box<dyn RespReply> = match self.values.len() {
            0 => Box::new(NullReply {}),
            1 => Box::new(SimpleStringReply {
                value: self.values[0].clone(),
            }),
            _ => {
                let mut rs: Vec<Box<dyn RespReply>> = Vec::new();
                for vv in &self.values {
                    rs.push(Box::new(BulkStringReply { value: vv.clone() }))
                }
                Box::new(ArrayReply { values: rs })
            }
        };
        v.serialise()
    }
}
