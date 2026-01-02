use axum::{
    body::Body,
    extract::State,
    http::{header, StatusCode},
    response::Response,
    Json,
};
use metrics::counter;
use serde::{Deserialize, Serialize};
use urlencoding::encode;
use tracing::info;
use utoipa::ToSchema;

use crate::errors::AppError;
use crate::models::{ApiResponse, ExcelDsl};
use crate::services::{ExcelGenerator, FileStorage};

#[derive(Clone)]
pub struct AppState {
    pub storage: FileStorage,
}

/// 直接生成 Excel 并返回二进制流
#[utoipa::path(
    post,
    path = "/api/excel/generate",
    request_body = ExcelDsl,
    responses(
        (status = 200, description = "Excel 文件二进制流", content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
    ),
    tag = "Excel 生成"
)]
pub async fn generate_excel(
    Json(dsl): Json<ExcelDsl>,
) -> Result<Response, AppError> {
    info!("直接生成 Excel 文件: {}", dsl.filename);
    counter!("api.excel.generate.total").increment(1);
    
    // 生成 Excel
    let mut generator = ExcelGenerator::new();
    let data = generator.generate(&dsl)?;
    
    counter!("api.excel.generate.success").increment(1);
    
    // 返回二进制流
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .header(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", dsl.filename))
        .body(Body::from(data))
        .map_err(|e| AppError::InternalError(e.to_string()))?;
    
    Ok(response)
}

/// 异步生成请求
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AsyncGenerateResponse {
    /// 文件 ID
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub file_id: String,
}

/// 异步生成 Excel，返回文件 ID
#[utoipa::path(
    post,
    path = "/api/excel/async",
    request_body = ExcelDsl,
    responses(
        (status = 200, description = "统一返回格式", body = ApiResponse<AsyncGenerateResponse>,
            example = json!({
                "code": 0,
                "message": "success",
                "data": {"file_id": "550e8400-e29b-41d4-a716-446655440000"},
                "success": true
            })
        )
    ),
    tag = "Excel 生成"
)]
pub async fn generate_excel_async(
    State(state): State<AppState>,
    Json(dsl): Json<ExcelDsl>,
) -> Result<Json<ApiResponse<AsyncGenerateResponse>>, AppError> {
    info!("异步生成 Excel 文件: {}", dsl.filename);
    counter!("api.excel.async.total").increment(1);
    
    let filename = dsl.filename.clone();
    
    // 生成 Excel
    info!("[生成] 开始生成 Excel - filename: {}", filename);
    let mut generator = ExcelGenerator::new();
    let data = generator.generate(&dsl)?;
    info!("[生成] Excel 生成完成 - filename: {}, size: {} bytes", filename, data.len());
    
    // 存储文件
    info!("[生成] 调用存储服务 - filename: {}", filename);
    let file_id = state.storage.store(filename.clone(), data).await?;
    info!("[生成] 文件存储成功 - file_id: {}, filename: {}", file_id, filename);
    
    counter!("api.excel.async.success").increment(1);
    
    Ok(Json(ApiResponse::success(AsyncGenerateResponse { file_id })))
}

/// 下载请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct DownloadRequest {
    /// 文件 ID
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub file_id: String,
}

/// 根据文件 ID 下载 Excel 文件（POST 方法）
#[utoipa::path(
    post,
    path = "/api/excel/download",
    request_body = DownloadRequest,
    responses(
        (status = 200, description = "Excel 文件二进制流", content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
    ),
    tag = "Excel 生成"
)]
pub async fn download_excel(
    State(state): State<AppState>,
    Json(req): Json<DownloadRequest>,
) -> Result<Response, AppError> {
    info!("下载 Excel 文件 (POST): {}", req.file_id);
    counter!("api.excel.download.total").increment(1);
    
    // 获取文件
    let (filename, data) = state.storage.retrieve(&req.file_id).await?;
    
    counter!("api.excel.download.success").increment(1);
    
    // 编码文件名以支持中文（RFC 5987）
    let encoded_filename = encode(&filename);
    let ascii_filename = if filename.chars().all(|c| c.is_ascii()) {
        &filename
    } else {
        "download.xlsx"
    };
    let content_disposition = format!(
        "attachment; filename=\"{}\"; filename*=UTF-8''{}",
        ascii_filename,
        encoded_filename
    );
    
    // 返回二进制流
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .header(header::CONTENT_DISPOSITION, content_disposition)
        .body(Body::from(data))
        .map_err(|e| AppError::InternalError(e.to_string()))?;
    
    Ok(response)
}

/// 根据文件 ID 下载 Excel 文件（GET 方法）
#[utoipa::path(
    get,
    path = "/api/excel/download/{file_id}",
    params(
        ("file_id" = String, Path, description = "文件 ID")
    ),
    responses(
        (status = 200, description = "Excel 文件二进制流", content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
        (status = 200, description = "文件不存在或已过期", body = ApiResponse<()>,
            example = json!({
                "code": 1003,
                "message": "文件不存在",
                "data": null,
                "success": false
            })
        )
    ),
    tag = "Excel 生成"
)]
pub async fn download_excel_get(
    State(state): State<AppState>,
    axum::extract::Path(file_id): axum::extract::Path<String>,
) -> Result<Response, AppError> {
    info!("[下载-GET] 收到下载请求 - file_id: {}", file_id);
    counter!("api.excel.download_get.total").increment(1);
    
    // 获取文件
    info!("[下载-GET] 调用存储服务检索文件 - file_id: {}", file_id);
    let (filename, data) = state.storage.retrieve(&file_id).await?;
    info!("[下载-GET] 文件检索成功 - file_id: {}, filename: {}, size: {} bytes", file_id, filename, data.len());
    
    counter!("api.excel.download_get.success").increment(1);
    
    // 编码文件名以支持中文（RFC 5987）
    let encoded_filename = encode(&filename);
    let content_disposition = format!(
        "attachment; filename=\"{}\"; filename*=UTF-8''{}",
        if filename.chars().all(|c| c.is_ascii()) { &filename } else { "download.xlsx" },
        encoded_filename
    );
    
    info!("[下载-GET] 返回文件流 - file_id: {}, filename: {}", file_id, filename);
    
    // 返回二进制流
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .header(header::CONTENT_DISPOSITION, content_disposition)
        .body(Body::from(data))
        .map_err(|e| AppError::InternalError(e.to_string()))?;
    
    Ok(response)
}

/// 健康检查
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "服务健康")
    ),
    tag = "系统"
)]
pub async fn health_check() -> &'static str {
    "OK"
}

/// 存储状态查询响应
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StorageStatusResponse {
    /// 当前存储的文件数量
    pub file_count: usize,
}

/// 查询存储状态
#[utoipa::path(
    post,
    path = "/api/excel/status",
    responses(
        (status = 200, description = "统一返回格式", body = ApiResponse<StorageStatusResponse>,
            example = json!({
                "code": 0,
                "message": "success",
                "data": {"file_count": 42},
                "success": true
            })
        )
    ),
    tag = "系统"
)]
pub async fn storage_status(
    State(state): State<AppState>,
) -> Json<ApiResponse<StorageStatusResponse>> {
    let count = state.storage.count().await;
    
    Json(ApiResponse::success(StorageStatusResponse {
        file_count: count,
    }))
}
