use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("failed to parse request at `{0}`. Details: {1}")]
    ParseRequestFailed(String, String),
    #[error("unsupported command `{0}`")]
    UnsupportedCommand(String),
    #[error("invalid command `{0}`. Details: {1}")]
    InvalidCommand(String, String),
    #[error("invalid command body. Details: {0}")]
    InvalidCommandBody(String),
    #[error("ERR value is not an integer or out of range")]
    InvalidIntValue,
    #[error("value is not a valid float")]
    InvalidFloatValue,
    #[error("value is out of range, must be positive")]
    InvalidNegValue,
    #[error("ERR wrong number of arguments for command")]
    IncorrectArgCount,
    #[error("unknown request error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("WRONGTYPE Operation against a key holding the wrong kind of value")]
    IncorrectType,
}

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("INTERNAL Internal error")]
    Error,
    #[error("INTERNAL Key already exists")]
    KeyError,
}

#[derive(Error, Debug)]
pub enum IncrCommandError {
    #[error("value is not an integer or out of range")]
    InvalidValue,
    #[error("increment or decrement would overflow")]
    ResultOverflow,
}

#[derive(Error, Debug)]
pub enum HIncrByCommandError {
    #[error("hash value is not an integer")]
    InvalidHashValue,
}
