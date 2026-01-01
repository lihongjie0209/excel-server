# 安装部署

## 环境要求

- **Rust**: 1.70+ (推荐 1.75+)
- **Cargo**: 随 Rust 安装
- **操作系统**: Linux / macOS / Windows

## 从源码安装

### 1. 克隆仓库

```bash
git clone https://github.com/yourusername/excel-server.git
cd excel-server
```

### 2. 编译项目

```bash
cargo build --release
```

编译后的二进制文件位于 `target/release/excel-server`

### 3. 运行服务

```bash
./target/release/excel-server
```

或 Windows:
```powershell
.\target\release\excel-server.exe
```

## 配置文件

### 创建配置

```bash
mkdir -p config
cp config/default.toml.example config/default.toml
```

### 编辑配置

```toml
# config/default.toml
[server]
host = "0.0.0.0"
port = 3000

[file_storage]
storage_dir = "./data/files"
ttl = 3600  # 文件过期时间（秒）

[logging]
level = "info"
```

## 运行模式

### 开发模式

```bash
cargo run
```

### 生产模式

```bash
cargo build --release
./target/release/excel-server
```

### 后台运行

#### systemd (Linux)

创建服务文件 `/etc/systemd/system/excel-server.service`:

```ini
[Unit]
Description=Excel Server
After=network.target

[Service]
Type=simple
User=excel-server
WorkingDirectory=/opt/excel-server
ExecStart=/opt/excel-server/excel-server
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

启动服务:
```bash
sudo systemctl daemon-reload
sudo systemctl enable excel-server
sudo systemctl start excel-server
```

#### Docker

创建 `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/excel-server /usr/local/bin/
COPY config /etc/excel-server/config
EXPOSE 3000
CMD ["excel-server"]
```

构建并运行:
```bash
docker build -t excel-server .
docker run -d -p 3000:3000 -v $(pwd)/data:/data excel-server
```

#### Docker Compose

创建 `docker-compose.yml`:

```yaml
version: '3.8'
services:
  excel-server:
    build: .
    ports:
      - "3000:3000"
    volumes:
      - ./data:/data
      - ./config:/etc/excel-server/config
    environment:
      - RUST_LOG=info
    restart: unless-stopped
```

运行:
```bash
docker-compose up -d
```

## 验证安装

### 检查服务状态

```bash
curl http://localhost:3000/health
```

应返回: `OK`

### 测试生成功能

```bash
curl -X POST http://localhost:3000/api/excel/generate \
  -H "Content-Type: application/json" \
  -d '{
    "sheets": [{
      "name": "Sheet1",
      "cells": [
        {"row": 0, "col": 0, "value": "Hello", "value_type": "string"}
      ]
    }]
  }' \
  --output test.xlsx
```

## 性能优化

### 生产环境建议

1. **调整文件描述符限制**:
```bash
ulimit -n 65535
```

2. **配置日志级别**:
```toml
[logging]
level = "warn"  # 生产环境使用 warn 或 error
```

3. **使用反向代理** (Nginx):
```nginx
upstream excel_server {
    server localhost:3000;
}

server {
    listen 80;
    server_name excel.example.com;

    location / {
        proxy_pass http://excel_server;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 下一步

- [配置说明](/guide/configuration) - 详细配置选项
- [快速开始](/guide/getting-started) - 开始使用
- [使用示例](/guide/examples) - 实际案例
