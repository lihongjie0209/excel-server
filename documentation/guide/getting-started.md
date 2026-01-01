# 快速开始

本指南将帮助你快速上手 Excel Server。

## 环境要求

- Rust 1.70+
- Cargo
- Windows/Linux/macOS

## 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/lihongjie0209/excel-server.git
cd excel-server

# 编译项目
cargo build --release

# 编译后的可执行文件位于
./target/release/excel-server
```

### 开发模式

```bash
# 开发模式运行（带调试信息）
cargo run

# Release 模式运行（优化性能）
cargo run --release
```

## 运行服务

### 启动服务

```bash
cargo run --release
```

服务默认监听 `http://localhost:3000`

### 验证服务

```bash
# 健康检查
curl http://localhost:3000/health

# 应返回: OK
```

```bash
# 查看 API 文档
open http://localhost:3000/swagger-ui/
```

## 第一个请求

### 1. 直接生成 Excel

最简单的方式，一次请求完成生成和下载：

```bash
curl -X POST http://localhost:3000/api/excel/generate \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {
      "title": "My First Excel"
    },
    "sheets": [{
      "name": "Sheet1",
      "cells": [
        {"r": 0, "c": 0, "type": "string", "value": "Hello"},
        {"r": 0, "c": 1, "type": "string", "value": "World"}
      ]
    }]
  }' \
  --output my-first-excel.xlsx
```

### 2. 异步生成 + 下载

适合大文件或需要异步处理的场景：

```bash
# 步骤 1: 提交生成任务
curl -X POST http://localhost:3000/api/excel/async \
  -H "Content-Type: application/json" \
  -d '{
    "properties": {"title": "Async Excel"},
    "sheets": [{"name": "Sheet1", "cells": []}]
  }'

# 响应示例:
# {
#   "code": 0,
#   "message": "success",
#   "data": {
#     "file_id": "550e8400-e29b-41d4-a716-446655440000"
#   },
#   "success": true
# }

# 步骤 2: 下载文件（使用 GET 方法）
curl -o my-excel.xlsx \
  http://localhost:3000/api/excel/download/550e8400-e29b-41d4-a716-446655440000
```

## 使用示例文件

项目提供了两个示例文件：

### 简单示例

```bash
curl -X POST http://localhost:3000/api/excel/generate \
  -H "Content-Type: application/json" \
  -d @examples/simple.json \
  --output simple.xlsx
```

### 高级示例

包含样式、公式、数据表格、条件格式等：

```bash
curl -X POST http://localhost:3000/api/excel/generate \
  -H "Content-Type: application/json" \
  -d @examples/advanced.json \
  --output advanced.xlsx
```

## 前端集成

### HTML 直接下载

```html
<a href="http://localhost:3000/api/excel/download/file-id-here" download>
  下载 Excel 文件
</a>
```

### JavaScript 下载

```javascript
async function generateAndDownload() {
  // 1. 生成文件
  const response = await fetch('http://localhost:3000/api/excel/async', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      properties: { title: 'My Report' },
      sheets: [{ name: 'Data', cells: [...] }]
    })
  });
  
  const { data } = await response.json();
  const fileId = data.file_id;
  
  // 2. 下载文件
  window.location.href = `http://localhost:3000/api/excel/download/${fileId}`;
}
```

## 配置服务

创建或编辑 `config/default.toml`：

```toml
[server]
host = "0.0.0.0"    # 监听地址
port = 3000         # 监听端口

[storage]
temp_dir = "./temp"          # 文件存储目录
max_age_seconds = 3600       # 文件保留 1 小时
```

重启服务使配置生效。

## 监控和调试

### 查看监控指标

```bash
curl http://localhost:3000/metrics
```

### 查看日志

服务启动时会输出日志到控制台：

```
2026-01-01T10:00:00.123Z INFO excel_server: 服务启动成功
2026-01-01T10:00:00.124Z INFO excel_server: 监听地址: 0.0.0.0:3000
2026-01-01T10:00:00.125Z INFO excel_server: Swagger UI: http://localhost:3000/swagger-ui/
```

## 下一步

- [配置说明](/guide/configuration) - 详细的配置选项
- [API 文档](/api/overview) - 完整的 API 接口说明
- [DSL 规范](/dsl/overview) - Excel DSL 详细语法
- [使用示例](/guide/examples) - 更多实际应用示例

## 故障排查

### 端口被占用

```bash
# Windows
netstat -ano | findstr :3000

# Linux/macOS
lsof -i :3000
```

修改配置文件中的端口或停止占用端口的进程。

### 文件生成失败

检查：
1. DSL 格式是否正确（可在 Swagger UI 中测试）
2. 查看服务日志输出的错误信息
3. 确认 temp 目录有写入权限

### 文件下载失败

检查：
1. file_id 是否正确
2. 文件是否已过期（默认 1 小时）
3. temp 目录是否存在

## 获取帮助

如遇到问题：

1. 查看 [常见问题](/guide/faq)
2. 搜索 [GitHub Issues](https://github.com/lihongjie0209/excel-server/issues)
3. 提交新的 Issue

