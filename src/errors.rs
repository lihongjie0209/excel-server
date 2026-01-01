use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

use crate::models::ApiResponse;

/// 业务错误枚举
#[derive(Debug, Error)]
pub enum AppError {
    #[error("参数错误: {0}")]
    ValidationError(String),
    
    #[error("资源不存在: {0}")]
    NotFound(String),
    
    #[error("Excel 生成失败: {0}")]
    ExcelGenerationError(String),
    
    #[error("文件存储错误: {0}")]
    StorageError(String),
    
    #[error("内部错误: {0}")]
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            AppError::ValidationError(msg) => (1001, msg),
            AppError::NotFound(msg) => (1003, msg),
            AppError::ExcelGenerationError(msg) => (2001, msg),
            AppError::StorageError(msg) => (2002, msg),
            AppError::InternalError(msg) => (5000, msg),
        };
        
        let body = ApiResponse::<()> {
            code,
            message,
            data: None,
            success: false,
        };
        
        // 注意：HTTP 状态码仍然是 200
        (StatusCode::OK, Json(body)).into_response()
    }
}

impl From<rust_xlsxwriter::XlsxError> for AppError {
    fn from(err: rust_xlsxwriter::XlsxError) -> Self {
        AppError::ExcelGenerationError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::StorageError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::StorageError(err.to_string())
    }
}
