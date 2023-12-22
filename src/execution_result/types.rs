use std::fmt::Display;

pub enum ResultType {
    SimpleString,
}

impl Display for ResultType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ResultType::SimpleString => "+",
        };
        write!(f, "{}", s)
    }
}
