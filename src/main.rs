mod config;
mod errors;
mod handlers;
mod models;
mod routes;
mod services;

use axum::extract::DefaultBodyLimit;
use axum::http::{header, Method};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::decompression::RequestDecompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::handlers::AppState;
use crate::routes::create_router;
use crate::services::FileStorage;

#[tokio::main]
async fn main() {
    // 初始化追踪
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "excel_server=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // 加载配置
    let config = Config::load().unwrap_or_else(|e| {
        tracing::warn!("配置加载失败，使用默认配置: {}", e);
        Config::default()
    });
    
    info!("服务配置: {:?}", config);
    
    // 初始化监控
    let prometheus_handle = setup_metrics_recorder();
    
    // 初始化文件存储
    let storage = FileStorage::new(
        config.storage.temp_dir.clone(),
        config.storage.max_age_seconds,
    )
    .expect("初始化文件存储失败");
    
    info!("文件存储已初始化: {:?}", config.storage.temp_dir);
    
    // 创建应用状态
    let state = AppState { storage };
    
    // 创建路由
    let app = create_router(state)
        .route("/metrics", axum::routing::get(move || async move {
            prometheus_handle.render()
        }))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(RequestDecompressionLayer::new()) // 解压缩请求体
                .layer(CompressionLayer::new()) // 压缩响应体
                .layer(DefaultBodyLimit::max(500 * 1024 * 1024)) // 500MB 限制
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST])
                        .allow_headers([header::CONTENT_TYPE, header::CONTENT_ENCODING, header::ACCEPT_ENCODING]),
                ),
        );
    
    // 绑定地址
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("绑定地址失败");
    
    info!("🚀 服务启动成功");
    info!("📍 监听地址: {}", addr);
    info!("📚 API 文档: http://{}/swagger-ui/", addr);
    info!("📖 在线文档: http://{}/docs/", addr);
    info!("🎮 性能测试: http://{}/demo", addr);
    info!("💊 健康检查: http://{}/health", addr);
    info!("📊 监控指标: http://{}/metrics", addr);
    
    // 启动服务
    axum::serve(listener, app)
        .await
        .expect("服务运行失败");
}

fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];
    
    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_request_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}
