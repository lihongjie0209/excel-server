use axum::{
    routing::{get, post},
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::*;
use crate::handlers::docs;
use crate::models::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        generate_excel,
        generate_excel_async,
        download_excel,
        download_excel_get,
        health_check,
        storage_status,
    ),
    components(
        schemas(
            ApiResponse<AsyncGenerateResponse>,
            ApiResponse<StorageStatusResponse>,
            ExcelDsl,
            DocumentProperties,
            Style,
            FontStyle,
            FillStyle,
            AlignStyle,
            BorderStyle,
            ProtectStyle,
            Worksheet,
            Cell,
            CellType,
            CellValue,
            RangeSpec,
            RangeCoords,
            Table,
            TableColumn,
            DataValidation,
            ConditionalFormat,
            Sparkline,
            LocationSpec,
            LocationCoords,
            AsyncGenerateResponse,
            DownloadRequest,
            StorageStatusResponse,
        )
    ),
    tags(
        (name = "Excel 生成", description = "Excel 文件生成相关接口"),
        (name = "系统", description = "系统监控和健康检查接口")
    ),
    info(
        title = "Excel Server API",
        version = "1.0.0",
        description = "基于 DSL 规范的 Excel 生成服务",
    )
)]
struct ApiDoc;

pub fn create_router(state: AppState) -> Router {
    // API 路由
    let api_routes = Router::new()
        .route("/excel/generate", post(generate_excel))
        .route("/excel/async", post(generate_excel_async))
        .route("/excel/download", post(download_excel))
        .route("/excel/download/:file_id", get(download_excel_get))
        .route("/excel/status", post(storage_status))
        .with_state(state);
    
    // 系统路由
    let system_routes = Router::new()
        .route("/health", get(health_check));
    
    // Swagger UI
    let swagger = SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi());
    
    // 文档路由
    let docs_routes = Router::new()
        .route("/docs", get(docs::serve_docs_root))
        .route("/docs/", get(docs::serve_docs_root))
        .route("/docs/*path", get(docs::serve_docs));
    
    // 组合所有路由
    Router::new()
        .nest("/api", api_routes)
        .merge(system_routes)
        .merge(swagger)
        .merge(docs_routes)
}
