# 更新日志 (Changelog)

## [v0.2.0] - 2026-01-01

### ✨ 新增功能
- 🚀 使用 `DashMap` 替代 `Arc<RwLock<HashMap>>`，提升并发性能
- 💾 实现文件系统持久化，服务重启后文件不丢失
- 📁 支持从文件系统自动加载已存在的文件
- 🔄 自动清理过期文件（内存和磁盘同步）

### 🔧 优化改进
- ⚡ 移除读写锁，采用无锁并发访问
- 📝 简化 API，无需显式 `.read().await` / `.write().await`
- 🗂️ 文件元数据独立存储（`.meta.json`）
- 🧹 优化代码，移除未使用的导入

### 📦 依赖更新
- ➕ 新增 `dashmap = "6.1"` - 高性能并发 HashMap
- ➕ 新增 `serde_json::Error` 错误处理

### 📄 文档更新
- 📚 新增 [docs/PERSISTENCE.md](docs/PERSISTENCE.md) - 持久化功能说明
- 🧪 新增 [examples/test_persistence.ps1](examples/test_persistence.ps1) - 持久化测试脚本
- 📝 更新 README.md，添加持久化功能说明

### 🧪 测试
- ✅ 所有 43 个单元测试通过
- ✅ Release 编译成功
- ✅ 持久化功能验证通过

---

## [v0.1.0] - 2025-12-31

### ✨ 初始版本
- 📊 实现完整的 Excel DSL v1.3 规范
- 🚀 支持两种生成模式：直接返回二进制流 / 异步生成 + 下载
- 📝 集成 OpenAPI 3.0 文档（Swagger UI）
- 📈 集成 Prometheus 监控指标
- 🔍 集成分布式追踪（tracing）
- 🧪 完整的单元测试覆盖（~85%）
- 📚 完整的项目文档

### 🎯 核心功能
- Excel 文件生成（基于 rust_xlsxwriter）
- 样式支持（字体、填充、对齐、边框、保护）
- 单元格类型（字符串、数字、公式、日期时间）
- 合并单元格
- 数据表格
- 数据验证
- 条件格式
- A1 引用和坐标系统

### 📦 技术栈
- Axum 0.7 - Web 框架
- rust_xlsxwriter 0.77 - Excel 生成
- utoipa 4 + utoipa-swagger-ui 7 - OpenAPI 文档
- metrics 0.22 + metrics-exporter-prometheus 0.15 - 监控
- tokio 1.48 - 异步运行时
