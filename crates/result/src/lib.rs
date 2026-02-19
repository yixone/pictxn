use std::error::Error;

use derive_more::Display;

#[derive(Debug, Display)]
pub enum AppError {
    /// Error not labeled
    NotLabeled,

    /// Internal Application Error
    InternalError {
        /// Source of error
        source: Box<dyn Error + Send + Sync>,
    },
}
