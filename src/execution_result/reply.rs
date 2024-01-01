use std::collections::HashMap;

pub trait RespReply {
    fn serialise(&self) -> String;
}

pub struct SimpleStringReply {
    pub value: String,
}

impl RespReply for SimpleStringReply {
    fn serialise(&self) -> String {
        format!("+{}\r\n", &self.value)
    }
}

pub struct SimpleErrorReply {
    pub message: String,
}

impl RespReply for SimpleErrorReply {
    fn serialise(&self) -> String {
        format!("-{}\r\n", &self.message)
    }
}

pub struct NullReply;
impl RespReply for NullReply {
    fn serialise(&self) -> String {
        "_\r\n".to_string()
    }
}

pub struct IntegerReply {
    pub value: i64,
}
impl RespReply for IntegerReply {
    fn serialise(&self) -> String {
        format!(":{}\r\n", self.value)
    }
}

pub struct BulkStringReply {
    pub value: String,
}
impl RespReply for BulkStringReply {
    fn serialise(&self) -> String {
        format!("${}\r\n{}\r\n", self.value.len(), &self.value)
    }
}

pub struct ArrayReply {
    pub values: Vec<Box<dyn RespReply>>,
}
impl RespReply for ArrayReply {
    fn serialise(&self) -> String {
        let mut res = format!("*{}\r\n", self.values.len());
        for v in &self.values {
            res += v.serialise().as_str();
        }
        res
    }
}

pub struct MapReply {
    pub values: HashMap<Box<dyn RespReply>, Box<dyn RespReply>>,
}

impl RespReply for MapReply {
    fn serialise(&self) -> String {
        let mut res = format!("%{}\r\n", self.values.len());
        for (k, v) in self.values.iter() {
            res += k.serialise().as_str();
            res += v.serialise().as_str();
        }
        res
    }
}
