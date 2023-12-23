use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("failed to parse request at `{0}`. Details: {1}")]
    ParseRequestFailed(String, String),
    #[error("unsupported command `{0}`")]
    UnsupportedCommand(String),
    #[error("invalid command `{0}`. Details: {1}")]
    InvalidCommand(String, String),
    #[error("unknown request error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum SetCommandError {
    #[error("invalid command body. Details: {0}")]
    InvalidBody(String),
}
