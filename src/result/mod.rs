/// Implementation of ActixError trait
mod actix;
/// Types of errors
pub mod errors;
/// Mapping errors to the common `AppError` type
mod mappers;

pub type Result<T> = std::result::Result<T, errors::AppError>;
