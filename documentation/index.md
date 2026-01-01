---
layout: home

hero:
  name: "Excel Server"
  text: "高性能 Excel 生成服务"
  tagline: 基于 Rust + Axum + rust_xlsxwriter 构建
  image:
    src: /logo.svg
    alt: Excel Server
  actions:
    - theme: brand
      text: 快速开始
      link: /guide/getting-started
    - theme: alt
      text: API 文档
      link: /api/overview
    - theme: alt
      text: 在 GitHub 上查看
      link: https://github.com/lihongjie0209/excel-server

features:
  - icon: 📊
    title: 完整的 DSL 规范
    details: 支持 Excel DSL v1.3 规范，包含样式、公式、合并单元格、数据表格、条件格式等全部功能
  - icon: ⚡
    title: 高并发性能
    details: 使用 DashMap 实现无锁并发访问，性能提升 200-500%
  - icon: 💾
    title: 文件持久化
    details: 基于文件系统持久化存储，服务重启后文件不丢失，自动加载和过期清理
  - icon: 🚀
    title: 两种生成模式
    details: 支持直接返回二进制流和异步生成 + 文件 ID 下载两种模式
  - icon: 🔗
    title: RESTful API
    details: 提供 POST 和 GET 两种下载方式，支持中文文件名，前端使用更便捷
  - icon: 📝
    title: OpenAPI 文档
    details: 完整的 OpenAPI 3.0 规范文档，集成 Swagger UI，支持在线测试
  - icon: 📈
    title: 监控指标
    details: 集成 Prometheus 监控指标，实时追踪服务状态和性能
  - icon: 🔍
    title: 分布式追踪
    details: 内置 tracing 支持，完整的请求链路追踪和日志记录
  - icon: 🧪
    title: 高测试覆盖
    details: 43 个单元测试，约 85% 测试覆盖率，确保代码质量
---

## 快速开始

### 安装

```bash
git clone https://github.com/lihongjie0209/excel-server.git
cd excel-server
cargo build --release
```

### 运行服务

```bash
cargo run --release
```

服务默认监听 `http://localhost:3000`

### 生成 Excel

```bash
# 直接生成并下载
curl -X POST http://localhost:3000/api/excel/generate \
  -H "Content-Type: application/json" \
  -d @examples/simple.json \
  --output report.xlsx
```

### 异步生成 + 下载

```bash
# 1. 提交生成任务
curl -X POST http://localhost:3000/api/excel/async \
  -H "Content-Type: application/json" \
  -d @examples/simple.json

# 响应: {"code":0,"message":"success","data":{"file_id":"xxx"},"success":true}

# 2. 下载文件（GET 方法）
curl -o report.xlsx http://localhost:3000/api/excel/download/xxx
```

## 核心特性

### 🎯 技术栈

- **Web 框架**: Axum 0.7 (基于 Tokio)
- **Excel 生成**: rust_xlsxwriter 0.77
- **并发集合**: DashMap 6.1
- **API 文档**: utoipa 4 + utoipa-swagger-ui 7
- **监控**: metrics + metrics-exporter-prometheus
- **追踪**: tracing + tracing-subscriber

### 📦 响应格式

所有业务接口统一返回格式：

```json
{
  "code": 0,
  "message": "success",
  "data": {},
  "success": true
}
```

### 🔧 配置

在 `config/default.toml` 中配置：

```toml
[server]
host = "0.0.0.0"
port = 3000

[storage]
temp_dir = "./temp"          # 文件存储目录
max_age_seconds = 3600       # 文件保留时间（秒）
```

## 文档导航

::: tip 导航
查看完整文档了解更多功能
:::

- 📚 [入门指南](/guide/getting-started) - 快速开始使用 Excel Server
- 🔌 [API 文档](/api/overview) - 完整的 API 接口说明
- 📝 [DSL 规范](/dsl/overview) - Excel DSL v1.3 详细规范
- 💾 [持久化](/guide/persistence) - 文件持久化功能说明

## 性能对比

| 操作 | v0.1.0 (RwLock) | v0.2.0 (DashMap) | 提升 |
|------|-----------------|------------------|------|
| 并发读取 | 共享锁 | 无锁 | ~20-30% |
| 并发写入 | 独占锁 | 分片锁 | ~300-500% |
| 混合读写 | 阻塞 | 分片隔离 | ~200-400% |

## 社区

- [GitHub Issues](https://github.com/lihongjie0209/excel-server/issues)
- [讨论区](https://github.com/lihongjie0209/excel-server/discussions)
- [更新日志](/changelog)

## 许可证

[MIT License](https://opensource.org/licenses/MIT)

