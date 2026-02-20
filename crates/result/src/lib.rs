use std::error::Error;

use derive_more::Display;

pub mod mappers;

pub type Result<T> = std::result::Result<T, AppError>;

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
