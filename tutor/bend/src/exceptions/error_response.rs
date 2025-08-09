use std::error::Error;
use std::fmt;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use crate::app_error::AppError;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error_message: String,
}

// Display trait 구현
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

// Error trait 구현
impl Error for ErrorResponse {
    fn description(&self) -> &str {
        &self.error_message
    }
}

