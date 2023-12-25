use std::fmt::Display;

pub enum ResultType {
    SimpleString,
    Integer,
}

impl Display for ResultType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResultType::SimpleString => "+",
            ResultType::Integer => ":",
        };
        write!(f, "{}", s)
    }
}
