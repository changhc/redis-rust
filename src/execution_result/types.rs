use std::fmt::Display;

pub enum ResultType {
    SimpleString,
    SimpleError,
    Null,
    Integer,
}

impl Display for ResultType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResultType::SimpleString => "+",
            ResultType::SimpleError => "-",
            ResultType::Null => "_",
            ResultType::Integer => ":",
        };
        write!(f, "{}", s)
    }
}
