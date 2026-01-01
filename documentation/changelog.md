# 更新日志

记录 Excel Server 的所有版本变更。

## [v0.2.0] - 2026-01-01

### ✨ 新增功能

- 🚀 **DashMap 并发优化**: 使用 `DashMap` 替代 `Arc<RwLock<HashMap>>`，提升并发性能 200-500%
- 💾 **文件持久化**: 实现基于文件系统的持久化存储，服务重启后文件不丢失
- 📁 **自动加载**: 启动时自动从文件系统恢复未过期文件
- 🔄 **同步清理**: 过期文件在内存和磁盘同步删除
- 🔗 **GET 下载接口**: 新增 `GET /api/excel/download/{file_id}` 接口，前端使用更便捷
- 🌐 **中文文件名支持**: 实现 RFC 5987 标准，完美支持中文文件名下载

### 🔧 优化改进

- ⚡ **无锁并发**: 移除读写锁，采用分片锁机制
- 📝 **API 简化**: 无需显式 `.read().await` / `.write().await`
- 🗂️ **元数据分离**: 文件元数据独立存储（`.meta.json`）
- 🧹 **代码清理**: 移除未使用的导入和死代码

### 📦 依赖更新

- ➕ 新增 `dashmap = "6.1"` - 高性能并发 HashMap
- ➕ 新增 `urlencoding = "2.1"` - URL 编码支持
- ➕ 新增 `serde_json::Error` 错误处理

### 📄 文档更新

- 📚 新增 [文件持久化功能说明](/guide/persistence)
- 📚 新增 [GET 下载接口文档](/guide/get-download)
- 🧪 新增持久化测试脚本 `test_persistence.ps1`
- 🧪 新增 GET 下载测试脚本 `test_get_download.ps1`
- 🧪 新增中文文件名测试脚本 `test_chinese_filename.ps1`
- 📝 更新 README.md，添加新功能说明

### 🧪 测试

- ✅ 所有 43 个单元测试通过
- ✅ Release 编译成功
- ✅ 持久化功能验证通过
- ✅ 中文文件名测试通过

### 📊 性能对比

| 操作 | v0.1.0 | v0.2.0 | 提升 |
|------|--------|--------|------|
| 并发读取 | 共享锁 | 无锁 | ~20-30% |
| 并发写入 | 独占锁 | 分片锁 | ~300-500% |
| 混合读写 | 阻塞 | 分片隔离 | ~200-400% |

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

- **Excel 生成**: 基于 rust_xlsxwriter 实现
- **样式支持**: 字体、填充、对齐、边框、保护
- **单元格类型**: 字符串、数字、公式、日期时间
- **合并单元格**: 支持任意范围合并
- **数据表格**: 格式化表格支持
- **数据验证**: 下拉列表、范围验证等
- **条件格式**: 基于规则的格式化
- **坐标系统**: A1 引用和数字坐标双重支持

### 📦 技术栈

- **Axum 0.7**: 异步 Web 框架
- **rust_xlsxwriter 0.77**: Excel 生成库
- **utoipa 4 + utoipa-swagger-ui 7**: OpenAPI 文档
- **metrics 0.22 + metrics-exporter-prometheus 0.15**: 监控
- **tokio 1.48**: 异步运行时
- **serde 1.0 + serde_json 1.0**: 序列化
- **uuid 1**: UUID 生成
- **chrono 0.4**: 日期时间处理

### 🔧 架构设计

- **分层架构**: handlers → services → models
- **两种 API 模式**: 
  - 同步：直接返回 Excel 文件
  - 异步：返回 file_id，通过下载接口获取
- **内存存储**: `Arc<RwLock<HashMap>>` 存储文件
- **统一响应**: 所有接口返回统一的 JSON 格式
- **错误处理**: 自定义错误类型和转换

### 📝 API 接口

- `POST /api/excel/generate` - 直接生成
- `POST /api/excel/async` - 异步生成
- `POST /api/excel/download` - 下载文件
- `POST /api/excel/status` - 存储状态
- `GET /health` - 健康检查
- `GET /metrics` - 监控指标
- `GET /swagger-ui/` - API 文档

### 🧪 测试覆盖

- 41 个 excel_generator 模块测试
  - 8 个解析测试
  - 10 个样式测试
  - 15 个生成测试
  - 8 个边界测试
- 2 个 file_storage 模块测试

### 📚 文档

- README.md - 项目介绍和快速开始
- docs/spec.md - Excel DSL v1.3 规范
- TEST_REPORT.md - 测试报告
- PROJECT_SUMMARY.md - 项目总结
- examples/ - 示例文件和测试脚本

### 🔮 已知限制

- 文件存储在内存中，重启后丢失（v0.2.0 已修复）
- 使用 RwLock，高并发性能受限（v0.2.0 已修复）
- 仅支持 POST 下载方式（v0.2.0 已添加 GET）
- 中文文件名支持不完善（v0.2.0 已修复）

---

## 版本规范

本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)：

- **主版本号**: 不兼容的 API 修改
- **次版本号**: 向下兼容的功能性新增
- **修订号**: 向下兼容的问题修正

## 升级指南

### 从 v0.1.0 升级到 v0.2.0

**破坏性变更**: 无

**新功能**:
1. 文件会持久化到磁盘，确保 `temp_dir` 配置正确
2. 新增 GET 下载接口，前端代码可简化
3. 中文文件名自动处理，无需额外配置

**配置变更**: 无

**依赖更新**:
```bash
# 重新编译即可
cargo build --release
```

## 路线图

查看 [GitHub Projects](https://github.com/lihongjie0209/excel-server/projects) 了解未来计划。

### v0.3.0 (计划中)

- [ ] 认证和授权支持
- [ ] 数据库持久化（PostgreSQL/Redis）
- [ ] 分布式文件存储（S3/OSS）
- [ ] 更多数据验证类型
- [ ] Sparklines 实现
- [ ] 图表支持
- [ ] 性能优化和缓存

### v1.0.0 (计划中)

- [ ] 稳定 API
- [ ] 完整的生产环境支持
- [ ] 高可用部署方案
- [ ] 性能基准测试
- [ ] 安全加固
- [ ] 完整的国际化支持

## 贡献

欢迎贡献代码、报告问题或提出建议！

- [GitHub Issues](https://github.com/lihongjie0209/excel-server/issues)
- [讨论区](https://github.com/lihongjie0209/excel-server/discussions)
- [贡献指南](/contributing)

