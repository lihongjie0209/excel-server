# 配置说明

Excel Server 使用 TOML 格式的配置文件。

## 配置文件位置

默认按以下顺序查找配置文件：

1. `./config/local.toml` (优先级最高)
2. `./config/default.toml`
3. 内置默认值

## 完整配置示例

```toml
# config/default.toml

[server]
# 监听地址
host = "0.0.0.0"
# 监听端口
port = 3000
# 请求体最大大小（字节）
max_body_size = 10485760  # 10MB

[file_storage]
# 文件存储目录
storage_dir = "./data/files"
# 文件过期时间（秒）
ttl = 3600  # 1 小时
# 清理间隔（秒）
cleanup_interval = 300  # 5 分钟

[logging]
# 日志级别: trace, debug, info, warn, error
level = "info"
# 日志格式: json, pretty
format = "pretty"

[cors]
# 是否启用 CORS
enabled = true
# 允许的来源（* 表示所有）
allow_origins = ["*"]
# 允许的方法
allow_methods = ["GET", "POST", "OPTIONS"]
# 允许的头
allow_headers = ["Content-Type"]

[metrics]
# 是否启用 Prometheus 指标
enabled = true
# 指标端点路径
endpoint = "/metrics"
```

## 配置项详解

### 服务器配置 [server]

#### host

监听地址。

- **类型**: String
- **默认值**: `"0.0.0.0"`
- **说明**: 
  - `"0.0.0.0"` - 监听所有网络接口
  - `"127.0.0.1"` - 仅本地访问
  - `"192.168.1.100"` - 监听特定 IP

#### port

监听端口。

- **类型**: Integer
- **默认值**: `3000`
- **说明**: 服务监听的 TCP 端口

#### max_body_size

请求体最大大小。

- **类型**: Integer (字节)
- **默认值**: `10485760` (10MB)
- **说明**: 限制单次请求的 JSON 大小

---

### 文件存储配置 [file_storage]

#### storage_dir

文件存储目录。

- **类型**: String
- **默认值**: `"./data/files"`
- **说明**: 
  - 相对路径相对于可执行文件
  - 建议使用绝对路径
  - 目录不存在会自动创建

#### ttl

文件过期时间。

- **类型**: Integer (秒)
- **默认值**: `3600` (1 小时)
- **说明**: 
  - 文件超过此时间自动删除
  - 设为 `0` 表示永不过期（不推荐）

#### cleanup_interval

清理间隔。

- **类型**: Integer (秒)
- **默认值**: `300` (5 分钟)
- **说明**: 后台清理过期文件的间隔

---

### 日志配置 [logging]

#### level

日志级别。

- **类型**: String
- **默认值**: `"info"`
- **可选值**: 
  - `"trace"` - 最详细
  - `"debug"` - 调试信息
  - `"info"` - 常规信息
  - `"warn"` - 警告
  - `"error"` - 仅错误

#### format

日志格式。

- **类型**: String
- **默认值**: `"pretty"`
- **可选值**:
  - `"pretty"` - 人类可读格式
  - `"json"` - JSON 格式（便于日志收集）

---

### CORS 配置 [cors]

#### enabled

是否启用 CORS。

- **类型**: Boolean
- **默认值**: `true`

#### allow_origins

允许的来源。

- **类型**: Array[String]
- **默认值**: `["*"]`
- **示例**:
```toml
allow_origins = ["https://example.com", "https://app.example.com"]
```

#### allow_methods

允许的 HTTP 方法。

- **类型**: Array[String]
- **默认值**: `["GET", "POST", "OPTIONS"]`

#### allow_headers

允许的请求头。

- **类型**: Array[String]
- **默认值**: `["Content-Type"]`

---

### 监控配置 [metrics]

#### enabled

是否启用 Prometheus 指标。

- **类型**: Boolean
- **默认值**: `true`

#### endpoint

指标端点路径。

- **类型**: String
- **默认值**: `"/metrics"`

## 环境变量覆盖

可通过环境变量覆盖配置文件：

```bash
# 覆盖端口
export SERVER_PORT=8080

# 覆盖存储目录
export FILE_STORAGE_STORAGE_DIR=/var/lib/excel-server

# 覆盖日志级别
export LOGGING_LEVEL=debug

./excel-server
```

命名规则: `<SECTION>_<KEY>` (全大写，下划线分隔)

## 配置示例

### 开发环境

```toml
[server]
host = "127.0.0.1"
port = 3000

[file_storage]
storage_dir = "./dev-data"
ttl = 600  # 10 分钟

[logging]
level = "debug"
format = "pretty"
```

### 生产环境

```toml
[server]
host = "0.0.0.0"
port = 3000
max_body_size = 52428800  # 50MB

[file_storage]
storage_dir = "/var/lib/excel-server/files"
ttl = 7200  # 2 小时
cleanup_interval = 600  # 10 分钟

[logging]
level = "warn"
format = "json"

[cors]
enabled = true
allow_origins = ["https://app.example.com"]
```

### Docker 环境

```toml
[server]
host = "0.0.0.0"
port = 3000

[file_storage]
storage_dir = "/data/files"  # 映射到 Docker volume
ttl = 3600

[logging]
level = "info"
format = "json"
```

## 下一步

- [快速开始](/guide/getting-started) - 开始使用
- [使用示例](/guide/examples) - 实际案例
- [性能优化](/guide/performance) - 优化建议
