use std::fmt::Display;

pub enum ResultType {
    SimpleString,
    SimpleError,
}

impl Display for ResultType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResultType::SimpleString => "+",
            ResultType::SimpleError => "-",
        };
        write!(f, "{}", s)
    }
}
