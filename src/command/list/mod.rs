mod push;
pub use push::PushCommand;
mod lpop;
pub use lpop::LpopCommand;
mod lrange;
pub use lrange::LrangeCommand;
mod llen;
pub use llen::LlenCommand;

#[derive(Debug)]
pub enum OperationDirection {
    LEFT,
    RIGHT,
}
