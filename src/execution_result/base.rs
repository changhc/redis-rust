pub trait ExecutionResult {
    fn to_string(&self) -> String;
    fn serialise(&self) -> String;
}

pub fn to_simple_string(value: &String) -> String {
    format!("+{}\r\n", value)
}

pub fn to_simple_error(value: &String) -> String {
    format!("-{}\r\n", value)
}

pub fn to_null() -> String {
    "_\r\n".to_string()
}

pub fn to_integer(value: &String) -> String {
    format!(":{}\r\n", value)
}

pub fn to_bulk_string(value: &String) -> String {
    format!("${}\r\n{}\r\n", value.len(), value)
}

pub fn to_array(values: &Vec<String>) -> String {
    let mut res = format!("*{}\r\n", values.len());
    for v in values {
        res += to_bulk_string(v).as_str();
    }
    res
}
