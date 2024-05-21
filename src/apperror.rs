use std::io;

use actix_web::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // NotFound 错误的描述
    #[error("not found")]
    NotFound,
    #[error("invalid input")]
    InvalidInput,
    #[error("mysql error occurred")]
    MysqlError(#[from] mysql::Error),
    #[error("unknown error occurred")]
    Unknown,
    #[error("I/O error occurred")]
    IoError(#[from] io::Error),
    #[error("Custom failed with str: {0}")]
    FailedWithStr(String),
    #[error("Custom failed with code: {0}")]
    FailedWithCode(i32),
}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::FailedWithStr(_) | AppError::FailedWithCode(_) => {
                StatusCode::SERVICE_UNAVAILABLE
            }
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InvalidInput => StatusCode::BAD_REQUEST,
            AppError::IoError(_) | AppError::MysqlError(_) | AppError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
