pub mod errors;
mod mappers;

pub type Result<T> = std::result::Result<T, errors::AppError>;
