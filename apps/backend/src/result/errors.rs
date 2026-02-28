use std::error::Error;

use derive_more::Display;

#[derive(Debug, Display)]
#[display(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppError {
    /// Error not labeled
    NotLabeled,

    /// The requested feed is empty or all feeds returned an error
    EmptyFeed,

    /// Specified entity was not found
    NotFound,

    /// Received input too large
    #[display("TOO_LARGE_INPUT")]
    TooLargeInput { received: u64, excepted: u64 },

    /// Internal Application Error
    #[display("INTERNAL_SERVER_ERROR")]
    InternalError {
        source: Box<dyn Error + Send + Sync>,
    },
}
