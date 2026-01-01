use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 统一响应结构
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    /// 业务状态码，0 表示成功
    #[schema(example = 0)]
    pub code: i32,
    
    /// 提示信息
    #[schema(example = "success")]
    pub message: String,
    
    /// 业务数据
    pub data: Option<T>,
    
    /// 操作是否成功
    #[schema(example = true)]
    pub success: bool,
}

impl<T> ApiResponse<T> {
    /// 成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
            success: true,
        }
    }
    
    /// 成功响应（无数据）
    pub fn success_without_data() -> ApiResponse<()> {
        ApiResponse {
            code: 0,
            message: "success".to_string(),
            data: None,
            success: true,
        }
    }
    
    /// 失败响应
    pub fn error(code: i32, message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            code,
            message: message.into(),
            data: None,
            success: false,
        }
    }
}
