mod base;
pub use base::ExecutionResult;
mod error;
pub use error::ErrorResult;
mod ping;
pub use ping::PingResult;
mod types;
pub use types::ResultType;
mod bulk_string;
pub use bulk_string::BulkStringResult;

pub mod list;
pub mod string;
