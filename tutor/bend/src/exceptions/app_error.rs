use actix_web::{error, http::StatusCode, HttpResponse, ResponseError, Result};
use serde::{Deserialize, Serialize};
use sqlx::error::Error as SQLxError;
use actix_web::error::Error;

use std::fmt;

use crate::error_response::ErrorResponse;

#[derive(Debug, Serialize, Deserialize)]
pub enum AppError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

impl AppError {
    pub fn error_response(&self) -> String {
        match self {
            AppError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            AppError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            AppError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            AppError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }

        }
    }
}

// implement trait
// 기본
impl std::error::Error for AppError {}
impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DBError(_msg)         => StatusCode::INTERNAL_SERVER_ERROR ,
            AppError::ActixError(_msg)      => StatusCode::INTERNAL_SERVER_ERROR ,
            AppError::InvalidInput(_msg)    => StatusCode::BAD_REQUEST,
            AppError::NotFound(_msg)        => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error_message: self.error_response(),
        })
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}
impl From<actix_web::error::Error> for AppError {
    fn from(err: actix_web::error::Error) -> Self {
        AppError::ActixError(err.to_string())
    }
}
impl From<SQLxError> for AppError {
    fn from(err: SQLxError) -> Self {
        AppError::DBError(err.to_string())
    }
}
