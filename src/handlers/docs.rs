use axum::{
    body::Body,
    extract::Path,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use rust_embed::RustEmbed;

/// 嵌入的文档静态文件
#[derive(RustEmbed)]
#[folder = "documentation/.vitepress/dist"]
pub struct DocsAssets;

/// 嵌入的静态资源文件（demo页面等）
#[derive(RustEmbed)]
#[folder = "static"]
pub struct StaticAssets;

/// 服务 Demo 页面
pub async fn serve_demo() -> impl IntoResponse {
    match StaticAssets::get("demo.html") {
        Some(content) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(Body::from(content.data.to_vec()))
            .unwrap(),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Demo page not found"))
            .unwrap(),
    }
}

/// 服务文档静态文件
pub async fn serve_docs(Path(path): Path<String>) -> impl IntoResponse {
    serve_file(&path).await
}

/// 服务文档根路径
pub async fn serve_docs_root() -> impl IntoResponse {
    serve_file("index.html").await
}

async fn serve_file(path: &str) -> impl IntoResponse {
    let path = if path.is_empty() || path == "/" {
        "index.html"
    } else {
        path
    };

    match DocsAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(content.data.to_vec()))
                .unwrap()
        }
        None => {
            // 如果文件不存在，尝试返回 index.html (用于 SPA 路由)
            if let Some(index) = DocsAssets::get("index.html") {
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(Body::from(index.data.to_vec()))
                    .unwrap()
            } else {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("404 Not Found"))
                    .unwrap()
            }
        }
    }
}
