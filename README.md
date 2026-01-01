# Excel Server

[![GitHub](https://img.shields.io/github/license/lihongjie0209/excel-server)](https://github.com/lihongjie0209/excel-server/blob/master/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Axum](https://img.shields.io/badge/axum-0.7-blue.svg)](https://github.com/tokio-rs/axum)

基于 Rust + Axum 的高性能 Excel 生成服务，支持 DSL 规范、文件持久化、内嵌 VitePress 文档。

📚 **[完整文档](https://github.com/lihongjie0209/excel-server/tree/master/documentation)** | 🚀 **[快速开始](#快速开始)** | 📖 **[API 文档](http://localhost:13000/swagger-ui/)** | 🔄 **[更新日志](./CHANGELOG.md)**

## 功能特性

- 📊 支持完整的 Excel DSL 规范 (v1.3)
- 🚀 两种生成模式：直接返回二进制流 / 异步生成 + 下载
- 💾 **文件持久化**：使用文件系统存储，服务重启不丢失
- ⚡ **高并发**：使用 DashMap 实现无锁并发访问（300-500% 性能提升）
- 📝 完整的 OpenAPI 3.0 文档
- 📈 Prometheus 监控指标
- 🔍 分布式追踪支持
- 🌐 **内嵌文档**：VitePress 文档编译到二进制，单文件部署

## 快速开始

### 克隆仓库

```bash
git clone https://github.com/lihongjie0209/excel-server.git
cd excel-server
```

### 构建并运行

```bash
# 构建文档（首次运行需要）
cd documentation
npm install
npm run docs:build
cd ..

# 运行服务
cargo run --releaseOpenAPI 文档
- `GET /docs/` - VitePress 在线文档（内嵌）
```

服务默认监听 `http://localhost:13000`

### API 端点

- `POST /api/excel/generate` - 直接生成并返回 Excel 文件
- `POST /api/excel/async` - 异步生成，返回文件 ID
- `POST /api/excel/download` - 通过文件 ID 下载（POST 方法）
- `GET /api/excel/download/:file_id` - 通过文件 ID 下载（GET 方法，前端友好）
- `POST /api/excel/status` - 查看存储状态
- `GET /health` - 健康检查
- `GET /metrics` - Prometheus 监控指标
- `GET /swagger-ui/` - API 文档

## API 示例

### 1. 直接生成 Excel

```bash13000/api/excel/generate \
  -H "Content-Type: application/json" \
  -d @examples/simple.json \
  --output report.xlsx
```

### 2. 异步生成 + 下载（POST 方法）

```bash
# 提交生成任务
curl -X POST http://localhost:13000/api/excel/async \
  -H "Content-Type: application/json" \
  -d @examples/simple.json

# 响应: {"code":0,"message":"success","data":{"file_id":"xxx"},"success":true}

# 下载文件（POST 方法）
curl -X POST http://localhost:13000/api/excel/download \
  -H "Content-Type: application/json" \
  -d '{"file_id":"xxx"}' \
  --output report.xlsx
```

### 3. 异步生成 + 下载（GET 方法，推荐前端使用）

```bash
# 提交生成任务
curl -X POST http://localhost:13000/api/excel/async \
  -H "Content-Type: application/json" \
  -d @examples/simple.json

# 响应: {"code":0,"message":"success","data":{"file_id":"xxx"},"success":true}

# 下载文件（GET 方法，直接通过 URL）
curl -o report.xlsx http://localhost:13000/api/excel/download/xxx
```

### 4. 前端使用示例

```html
<!-- HTML 直接下载 -->
<a href="http://localhost:13000/api/excel/download/{file_id}" download>
  下载 Excel 文件
</a>
```

```javascript
// JavaScript 下载
const fileId = 'xxx';
window.location.href = `http://localhost:13000/api/excel/download/${fileId}`;

// 或使用 fetch
fetch(`http://localhost:1
fetch(`http://localhost:3000/api/excel/download/${fileId}`)
  .then(res => res.blob())
  .then(blob => {
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'report.xlsx';
    a.click();
    URL.revokeObjectURL(url);
  });
```

## DSL 示例

```json
{
  "filename": "report.xlsx",
  "styles": {
    "header": {
      "font": { "bold": true, "color": "#FFFFFF" },
      "fill": { "color": "#4472C4" },
      "align": { "h": "center", "v": "vcenter" }
    }
  },
  "sheets": [
    {
      "name": "Sales",
      "cells": [
        { "r": 0, "c": 0, "type": "string", "value": "Product", "style": "header" },
        { "r": 1, "c": 0, "type": "string", "value": "Widget A" }
      ]
    }
  ]
}
```

## 配置

创建 `config/default.toml`:

```toml
[server]
host = "0.0.0.0"
port = 13000

[storage]
temp_dir = "./temp"          # 文件存储目录（持久化）
max_age_seconds = 3600       # 文件最大保留时间（秒）
```

### 文件持久化

服务使用文件系统持久化存储，确保重启后文件不丢失：

- **存储位置**: `./temp` 目录
- **文件格式**: 
  - `{file_id}.dat` - Excel 文件数据
  - `{file_id}.meta.json` - 文件元数据
- **自动加载**: 服务启动时自动从文件系统恢复未过期文件
- **过期清理**: 自动清理超过 `max_age_seconds` 的文件

详细说明请参阅 [docs/PERSISTENCE.md](https://github.com/lihongjie0209/excel-server/blob/master/docs/PERSISTENCE.md)

## 开发

```bash
# 运行测试
cargo test

# 测试持久化功能
.\examples\test_persistence.ps1

# 查看文档
cargo doc --open
```

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](https://github.com/lihongjie0209/excel-server/blob/master/LICENSE) 文件。
