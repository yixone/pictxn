use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use tracing::error;

use crate::result::errors::AppError;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::TooLargeInput { .. } => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let code = self.status_code();

        if code.is_server_error() {
            error!(err = ?self, "Internal server error occured");
        }

        let res = ErrorResponse {
            code: code.as_u16(),
            message: self.to_string(),
        };

        HttpResponse::build(code).json(res)
    }
}
