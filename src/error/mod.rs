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
    #[error("unknown request error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum IncrCommandError {
    #[error("value is not an integer or out of range")]
    InvalidValue,
}

#[derive(Error, Debug)]
pub enum IncrCommandError {
    #[error("invalid command body. Details: {0}")]
    InvalidBody(String),
    #[error("value is not an integer or out of range")]
    InvalidValue,
}
