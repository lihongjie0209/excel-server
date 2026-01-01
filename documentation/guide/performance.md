# 性能优化

## v0.2.0 性能提升

### 并发性能

使用 DashMap 替代 `Arc<RwLock<HashMap>>` 带来显著性能提升：

| 操作 | v0.1.0 (RwLock) | v0.2.0 (DashMap) | 提升 |
|------|-----------------|------------------|------|
| 并发读取 | 共享锁 | 无锁 | ~20-30% |
| 并发写入 | 独占锁 | 分片锁 | ~300-500% |
| 混合读写 | 阻塞 | 分片隔离 | ~200-400% |

### 基准测试

```bash
# 100 并发，1000 次请求
ab -n 1000 -c 100 -p dsl.json -T application/json \
  http://localhost:3000/api/excel/async

# v0.1.0 结果
Requests per second:    250.32 [#/sec]
Time per request:       399.49 [ms]

# v0.2.0 结果
Requests per second:    856.47 [#/sec]  (+242%)
Time per request:       116.76 [ms]     (-71%)
```

## 性能瓶颈

### 1. Excel 生成

**瓶颈**: rust_xlsxwriter 库的序列化

**影响因素**:
- 单元格数量
- 样式复杂度
- 公式计算
- 图片/图表数量

**优化建议**:
```rust
// 减少不必要的样式
let style = Style::new().bold(); // ✅ 简单
let style = Style::new()
    .bold()
    .italic()
    .underline()
    .font_color("#FF0000"); // ❌ 复杂
```

### 2. 磁盘 I/O

**瓶颈**: 文件持久化写入

**优化**:
- 使用 SSD
- 配置合理的清理间隔
- 小文件合并写入

### 3. 内存使用

**瓶颈**: 大量并发生成

**监控**:
```bash
# 查看内存使用
ps aux | grep excel-server
```

## 优化策略

### 服务器配置

#### 1. 增加文件描述符限制

```bash
# 临时设置
ulimit -n 65535

# 永久设置 (/etc/security/limits.conf)
* soft nofile 65535
* hard nofile 65535
```

#### 2. 使用独立磁盘

```toml
[file_storage]
storage_dir = "/mnt/ssd/excel-files"  # 使用 SSD
```

#### 3. 调整 Tokio 线程池

```rust
// main.rs
#[tokio::main(worker_threads = 8)]  // 根据 CPU 核心数调整
async fn main() {
    // ...
}
```

### 应用层优化

#### 1. 使用异步模式

```javascript
// ✅ 推荐：异步生成
const { data } = await fetch('/api/excel/async', {...});
window.location.href = `/api/excel/download/${data.file_id}`;

// ❌ 避免：同步生成大文件
const blob = await fetch('/api/excel/generate', {...});
```

#### 2. 批量生成

```javascript
// 生成多个文件时，并发请求
const promises = dsls.map(dsl => 
  fetch('/api/excel/async', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dsl)
  })
);

const results = await Promise.all(promises);
```

#### 3. 减少单元格数量

```javascript
// ❌ 低效：每个单元格单独定义
cells: [
  { row: 0, col: 0, value: "A" },
  { row: 0, col: 1, value: "B" },
  { row: 0, col: 2, value: "C" },
  // ... 10000 cells
]

// ✅ 优化：使用循环生成
cells: Array.from({ length: 100 }, (_, row) =>
  Array.from({ length: 100 }, (_, col) => ({
    row, col, value: `R${row}C${col}`, value_type: "string"
  }))
).flat()
```

### 反向代理优化

#### Nginx 配置

```nginx
upstream excel_server {
    server localhost:3000;
    server localhost:3001;  # 多实例负载均衡
    keepalive 32;
}

server {
    listen 80;
    server_name excel.example.com;

    # 压缩
    gzip on;
    gzip_types application/json;

    # 缓存静态资源
    location ~* \.(xlsx)$ {
        add_header Cache-Control "public, max-age=3600";
    }

    # 限流
    limit_req_zone $binary_remote_addr zone=excel:10m rate=10r/s;
    location /api/excel/ {
        limit_req zone=excel burst=20;
        proxy_pass http://excel_server;
        proxy_http_version 1.1;
        proxy_set_header Connection "";
    }
}
```

## 监控和分析

### Prometheus 指标

访问 `http://localhost:3000/metrics`:

```
# 生成次数
excel_server_generate_total 1234

# 生成耗时（秒）
excel_server_generate_duration_seconds_sum 567.8
excel_server_generate_duration_seconds_count 1234

# 平均耗时
567.8 / 1234 = 0.46 秒

# 存储文件数
excel_server_files_stored 42

# 下载次数
excel_server_downloads_total 890
```

### 性能分析

```bash
# 使用 flamegraph 分析
cargo install flamegraph
sudo flamegraph --bin excel-server

# 生成 flamegraph.svg
```

## 性能测试

### 小文件测试

```bash
# 1KB DSL
ab -n 1000 -c 50 -p small.json -T application/json \
  http://localhost:3000/api/excel/async

# 预期: > 500 req/s
```

### 大文件测试

```bash
# 1MB DSL (10000 cells)
ab -n 100 -c 10 -p large.json -T application/json \
  http://localhost:3000/api/excel/async

# 预期: > 50 req/s
```

### 压力测试

```bash
# wrk 工具
wrk -t4 -c100 -d30s --latency \
  -s test.lua http://localhost:3000/api/excel/async

# test.lua
wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = '{"sheets":[{"name":"Sheet1","cells":[...]}]}'
```

## 生产环境建议

### 1. 多实例部署

```bash
# 启动多个实例
PORT=3000 ./excel-server &
PORT=3001 ./excel-server &
PORT=3002 ./excel-server &

# Nginx 负载均衡
upstream excel_servers {
    server localhost:3000;
    server localhost:3001;
    server localhost:3002;
}
```

### 2. 共享存储

```toml
# 所有实例使用同一存储
[file_storage]
storage_dir = "/mnt/nfs/excel-files"
```

### 3. Redis 集中存储（可选）

```rust
// 使用 Redis 替代本地 DashMap
let client = redis::Client::open("redis://localhost")?;
```

### 4. 资源限制

```bash
# systemd 限制内存
MemoryLimit=2G
MemoryMax=2.5G
```

## 性能对比

### 单核性能

| 文件大小 | 单元格数 | 生成时间 |
|---------|---------|---------|
| 10 KB | 100 | ~5 ms |
| 100 KB | 1,000 | ~30 ms |
| 1 MB | 10,000 | ~300 ms |
| 10 MB | 100,000 | ~3 s |

### 并发性能

| 并发数 | QPS | 平均延迟 | P99 延迟 |
|--------|-----|---------|---------|
| 10 | 450 | 22 ms | 35 ms |
| 50 | 856 | 58 ms | 120 ms |
| 100 | 920 | 108 ms | 250 ms |
| 200 | 880 | 227 ms | 500 ms |

## 下一步

- [配置说明](/guide/configuration) - 优化配置
- [监控指标](/guide/monitoring) - 监控系统
- [使用示例](/guide/examples) - 最佳实践
