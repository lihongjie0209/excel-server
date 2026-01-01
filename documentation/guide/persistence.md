# 文件持久化

从 v0.2.0 开始，Excel Server 支持文件持久化功能，确保服务重启后文件不丢失。

## 工作原理

### 双存储模式

1. **内存存储**: 使用 `DashMap` 快速访问
2. **磁盘存储**: 持久化保存，防止丢失

### 文件结构

每个生成的文件会创建两个磁盘文件：

```
data/files/
├── 550e8400-e29b-41d4-a716-446655440000.dat           # 二进制数据
└── 550e8400-e29b-41d4-a716-446655440000.meta.json     # 元数据
```

#### .dat 文件

存储 Excel 文件的二进制内容。

#### .meta.json 文件

存储文件元数据：

```json
{
  "file_id": "550e8400-e29b-41d4-a716-446655440000",
  "filename": "report.xlsx",
  "content_type": "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
  "size": 12345,
  "created_at": 1735689600,
  "expires_at": 1735693200
}
```

## 自动加载

服务启动时自动从磁盘加载所有未过期的文件到内存：

```rust
// 启动日志
[INFO] Loading files from filesystem...
[INFO] Loaded 5 files from filesystem
[INFO] Server listening on 0.0.0.0:3000
```

## 过期清理

### 自动清理

后台任务定期清理过期文件（内存 + 磁盘）：

- **检查间隔**: 5 分钟（可配置）
- **清理内容**: 
  - 从内存中移除
  - 删除 `.dat` 文件
  - 删除 `.meta.json` 文件

### 配置清理间隔

```toml
# config/default.toml
[file_storage]
cleanup_interval = 300  # 秒
```

## 配置文件 TTL

```toml
[file_storage]
ttl = 7200  # 2 小时
```

常用 TTL 设置：

| 场景 | TTL | 说明 |
|------|-----|------|
| 开发测试 | 600 | 10 分钟 |
| 临时导出 | 1800 | 30 分钟 |
| 标准场景 | 3600 | 1 小时（默认） |
| 长期保存 | 86400 | 24 小时 |
| 分享链接 | 604800 | 7 天 |

## 存储管理

### 查看存储状态

```bash
curl -X POST http://localhost:3000/api/excel/status \
  -H "Content-Type: application/json"
```

响应：

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

### 磁盘空间监控

```bash
# Linux/macOS
du -sh data/files/

# Windows
Get-ChildItem data\files\ | Measure-Object -Property Length -Sum
```

### 手动清理

```bash
# 清理所有文件
rm -rf data/files/*

# 清理特定文件
rm data/files/550e8400-e29b-41d4-a716-446655440000.*
```

## 性能影响

### 写入性能

| 操作 | v0.1.0 (仅内存) | v0.2.0 (持久化) | 性能损失 |
|------|----------------|----------------|---------|
| 生成小文件 (< 100KB) | ~5ms | ~10ms | ~2x |
| 生成中文件 (1-10MB) | ~50ms | ~80ms | ~1.6x |
| 生成大文件 (> 10MB) | ~500ms | ~550ms | ~1.1x |

**结论**: 对于大文件，磁盘 I/O 开销相对较小。

### 读取性能

- **首次读取**: 从内存读取，< 1ms
- **重启后**: 自动加载到内存，仍然 < 1ms

## 故障恢复

### 服务重启

```bash
# 停止服务
kill <PID>

# 重启服务
./excel-server
```

**结果**: 所有未过期的文件自动恢复。

### 磁盘故障

如果 `.dat` 或 `.meta.json` 文件损坏：

1. 服务跳过该文件并记录错误日志
2. 其他文件正常加载
3. 客户端重新生成受影响的文件

### 日志示例

```log
[WARN] Failed to load file 550e8400-...: metadata not found
[INFO] Loaded 4 files from filesystem (1 failed)
```

## 生产环境建议

### 1. 使用独立磁盘

```toml
[file_storage]
storage_dir = "/mnt/excel-storage/files"
```

### 2. 定期备份

```bash
# 每天备份
0 2 * * * tar -czf /backup/excel-files-$(date +\%Y\%m\%d).tar.gz /var/lib/excel-server/files
```

### 3. 监控磁盘空间

```bash
# 磁盘使用率超过 80% 时告警
df -h | grep /mnt/excel-storage | awk '{print $5}' | sed 's/%//' | \
  awk '{if ($1 > 80) print "WARN: Disk usage is " $1 "%"}'
```

### 4. 设置合理的 TTL

```toml
[file_storage]
# 根据业务场景调整
ttl = 3600  # 1 小时通常足够
```

### 5. 清理策略

```bash
# 清理超过 7 天的旧文件
find /var/lib/excel-server/files -type f -mtime +7 -delete
```

## 迁移到持久化版本

从 v0.1.0 升级到 v0.2.0：

1. **备份数据**（如果有）
2. **更新代码**: `git pull && cargo build --release`
3. **配置存储目录**:
```toml
[file_storage]
storage_dir = "./data/files"
ttl = 3600
```
4. **重启服务**: `./target/release/excel-server`

**注意**: v0.1.0 的内存数据无法迁移，需重新生成。

## 下一步

- [性能优化](/guide/performance) - 提升性能
- [配置说明](/guide/configuration) - 自定义配置
- [监控指标](/guide/monitoring) - 监控系统状态
