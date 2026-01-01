# 系统接口

Excel Server 提供系统管理和监控接口。

## 健康检查

### GET /health

检查服务是否正常运行。

**请求**:
```bash
curl http://localhost:3000/health
```

**响应**:
```
OK
```

- **HTTP 状态码**: 200
- **Content-Type**: text/plain

**用途**:
- 负载均衡器健康检查
- 监控系统探活
- CI/CD 部署验证

## 存储状态

### POST /api/excel/status

查询当前存储的文件数量。

**请求**:
```bash
curl -X POST http://localhost:3000/api/excel/status \
  -H "Content-Type: application/json"
```

**响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "file_count": 5
  },
  "success": true
}
```

## Prometheus 监控

### GET /metrics

导出 Prometheus 格式的监控指标。

**请求**:
```bash
curl http://localhost:3000/metrics
```

**响应**:
```
# HELP excel_server_generate_total Total number of Excel generations
# TYPE excel_server_generate_total counter
excel_server_generate_total 1234

# HELP excel_server_generate_duration_seconds Time spent generating Excel files
# TYPE excel_server_generate_duration_seconds histogram
excel_server_generate_duration_seconds_sum 567.8
excel_server_generate_duration_seconds_count 1234
excel_server_generate_duration_seconds_bucket{le="0.01"} 234
excel_server_generate_duration_seconds_bucket{le="0.1"} 890
excel_server_generate_duration_seconds_bucket{le="1.0"} 1200
excel_server_generate_duration_seconds_bucket{le="+Inf"} 1234

# HELP excel_server_files_stored Current number of files stored
# TYPE excel_server_files_stored gauge
excel_server_files_stored 42

# HELP excel_server_downloads_total Total number of downloads
# TYPE excel_server_downloads_total counter
excel_server_downloads_total{method="get"} 678
excel_server_downloads_total{method="post"} 212
```

### 可用指标

| 指标名称 | 类型 | 说明 |
|---------|------|------|
| `excel_server_generate_total` | Counter | 生成请求总数 |
| `excel_server_generate_duration_seconds` | Histogram | 生成耗时分布 |
| `excel_server_files_stored` | Gauge | 当前存储文件数 |
| `excel_server_downloads_total` | Counter | 下载请求总数 |
| `excel_server_download_errors_total` | Counter | 下载失败总数 |

### Prometheus 配置

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'excel-server'
    static_configs:
      - targets: ['localhost:3000']
    metrics_path: '/metrics'
    scrape_interval: 15s
```

### Grafana 示例查询

```promql
# 每秒生成请求数
rate(excel_server_generate_total[1m])

# 平均生成耗时
rate(excel_server_generate_duration_seconds_sum[5m]) 
/ 
rate(excel_server_generate_duration_seconds_count[5m])

# P99 生成耗时
histogram_quantile(0.99, 
  rate(excel_server_generate_duration_seconds_bucket[5m])
)

# 当前存储文件数
excel_server_files_stored

# 下载成功率
rate(excel_server_downloads_total[5m]) 
/ 
(rate(excel_server_downloads_total[5m]) + rate(excel_server_download_errors_total[5m]))
```

## OpenAPI 文档

### GET /swagger-ui/

访问交互式 API 文档（Swagger UI）。

**URL**: http://localhost:3000/swagger-ui/

**功能**:
- 在线测试所有 API
- 查看请求/响应示例
- 生成客户端代码

### GET /api-docs/openapi.json

获取 OpenAPI 3.0 规范文件。

**请求**:
```bash
curl http://localhost:3000/api-docs/openapi.json
```

**用途**:
- 生成客户端 SDK
- API 文档生成
- 自动化测试

## Docker 健康检查

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/excel-server /usr/local/bin/
EXPOSE 3000

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

CMD ["excel-server"]
```

## Kubernetes 探针

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: excel-server
spec:
  containers:
  - name: excel-server
    image: excel-server:latest
    ports:
    - containerPort: 3000
    
    # 存活探针
    livenessProbe:
      httpGet:
        path: /health
        port: 3000
      initialDelaySeconds: 5
      periodSeconds: 10
    
    # 就绪探针
    readinessProbe:
      httpGet:
        path: /health
        port: 3000
      initialDelaySeconds: 3
      periodSeconds: 5
```

## 监控告警

### Prometheus 告警规则

```yaml
# alerts.yml
groups:
  - name: excel-server
    rules:
      # 服务下线
      - alert: ExcelServerDown
        expr: up{job="excel-server"} == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Excel Server 服务下线"
      
      # 高错误率
      - alert: HighErrorRate
        expr: |
          rate(excel_server_download_errors_total[5m]) 
          / 
          rate(excel_server_downloads_total[5m]) > 0.1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "下载错误率超过 10%"
      
      # 文件堆积
      - alert: TooManyFiles
        expr: excel_server_files_stored > 1000
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "存储文件数超过 1000"
```

## 日志查询

### 结构化日志

```json
{
  "timestamp": "2026-01-01T12:00:00Z",
  "level": "INFO",
  "message": "Excel generated",
  "file_id": "550e8400-e29b-41d4-a716-446655440000",
  "size": 12345,
  "duration_ms": 45
}
```

### 常用查询

```bash
# 错误日志
grep "ERROR" logs/excel-server.log

# 慢请求（> 1s）
grep "duration_ms.*[0-9]{4,}" logs/excel-server.log

# 特定文件
grep "550e8400-e29b-41d4-a716-446655440000" logs/excel-server.log
```

## 下一步

- [性能优化](/guide/performance) - 优化系统性能
- [配置说明](/guide/configuration) - 自定义配置
- [API 概览](/api/overview) - 所有 API 接口
