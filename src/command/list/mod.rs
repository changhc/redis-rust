mod push;
pub use push::PushCommand;
mod pop;
pub use pop::PopCommand;
mod lrange;
pub use lrange::LrangeCommand;
mod llen;
pub use llen::LlenCommand;

#[derive(Debug)]
pub enum OperationDirection {
    LEFT,
    RIGHT,
}
