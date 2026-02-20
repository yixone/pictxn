use crate::AppError;

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::InternalError {
            source: Box::new(value),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        AppError::InternalError {
            source: Box::new(value),
        }
    }
}
