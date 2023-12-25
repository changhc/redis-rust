mod base;
pub use base::ExecutionResult;
mod ping;
pub use ping::PingResult;
mod set;
pub use set::SetResult;
mod get;
pub use get::GetResult;
mod int_op;
pub use int_op::IntOpResult;
mod types;
pub use types::ResultType;
