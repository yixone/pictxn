use crate::result::errors::AppError;

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::NotFound => AppError::NotFound,
            _ => AppError::InternalError {
                source: Box::new(value),
            },
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
