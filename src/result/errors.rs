use std::error::Error;

use derive_more::Display;

#[derive(Debug, Display)]
pub enum AppError {
    /// Error not labeled
    NotLabeled,

    /// The requested feed is empty or all feeds returned an error
    EmptyFeed,
    #[display("INVALID_PAGINATION_LIMIT")]
    InvalidPaginationLimit { received: u32, max: u32 },

    /// Specified entity was not found
    NotFound,

    /// Received input too large
    #[display("TOO_LARGE_INPUT")]
    TooLargeInput { received: u64, excepted: u64 },

    /// Internal Application Error
    InternalError {
        source: Box<dyn Error + Send + Sync>,
    },
}
