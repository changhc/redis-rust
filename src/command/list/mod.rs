mod push;
pub use push::PushCommand;
mod pop;
pub use pop::PopCommand;
mod lrange;
pub use lrange::LRangeCommand;
mod llen;
pub use llen::LLenCommand;

#[derive(Debug)]
pub enum OperationDirection {
    Left,
    Right,
}
