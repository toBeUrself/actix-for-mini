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
}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => {
                StatusCode::NOT_FOUND
            }
            AppError::InvalidInput => {
                StatusCode::BAD_REQUEST
            }
            AppError::MysqlError(_) | AppError::Unknown => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}