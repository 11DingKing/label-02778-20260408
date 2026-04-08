use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    TooManyRequests(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "{}", msg),
            AppError::Unauthorized(msg) => write!(f, "{}", msg),
            AppError::Forbidden(msg) => write!(f, "{}", msg),
            AppError::NotFound(msg) => write!(f, "{}", msg),
            AppError::Conflict(msg) => write!(f, "{}", msg),
            AppError::TooManyRequests(msg) => write!(f, "{}", msg),
            AppError::Internal(msg) => write!(f, "{}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (code, message) = match self {
            AppError::BadRequest(msg) => (400, msg.clone()),
            AppError::Unauthorized(msg) => (401, msg.clone()),
            AppError::Forbidden(msg) => (403, msg.clone()),
            AppError::NotFound(msg) => (404, msg.clone()),
            AppError::Conflict(msg) => (409, msg.clone()),
            AppError::TooManyRequests(msg) => (429, msg.clone()),
            AppError::Internal(msg) => {
                log::error!("Internal error: {}", msg);
                (500, "服务器内部错误".to_string())
            }
        };
        HttpResponse::build(actix_web::http::StatusCode::from_u16(code).unwrap())
            .json(serde_json::json!({"code": code, "message": message}))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        log::error!("Database error: {:?}", err);
        AppError::Internal(format!("数据库错误: {}", err))
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        log::error!("Bcrypt error: {:?}", err);
        AppError::Internal("密码处理错误".to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        log::error!("JWT error: {:?}", err);
        AppError::Unauthorized("认证失败".to_string())
    }
}
