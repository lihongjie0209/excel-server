# 文件持久化功能说明

## 🎯 功能概述

Excel Server 现在使用 **DashMap** 替代 `Arc<RwLock<HashMap>>`，并实现了**文件系统持久化**功能，确保服务重启后文件不会丢失。

## 📦 技术实现

### 1. 并发集合优化
- **之前**: `Arc<RwLock<HashMap<String, StoredFile>>>`
  - 读写锁性能开销较大
  - 高并发场景下存在锁竞争
  
- **现在**: `DashMap<String, StoredFile>`
  - 无锁并发访问（基于分片锁）
  - 更高的并发性能
  - 更简洁的 API（无需 `.read().await` / `.write().await`）

### 2. 文件持久化机制

#### 存储结构
```
temp/
├── {file_id}.dat         # 实际 Excel 文件数据
└── {file_id}.meta.json   # 文件元数据
```

#### 元数据格式
```json
{
  "file_id": "550e8400-e29b-41d4-a716-446655440000",
  "filename": "sales_report.xlsx",
  "created_timestamp": 1704067200
}
```

#### 工作流程

**存储文件时**:
1. 生成唯一的 UUID 作为 file_id
2. 将 Excel 数据写入 `{file_id}.dat`
3. 将元数据写入 `{file_id}.meta.json`
4. 同时保存到内存中（DashMap）

**服务启动时**:
1. 扫描 temp 目录下的所有 `.meta.json` 文件
2. 读取元数据并验证对应的 `.dat` 文件存在
3. 检查文件是否过期（基于 `created_timestamp` 和 `max_age_seconds`）
4. 将未过期的文件加载到内存（DashMap）
5. 删除已过期的文件

**删除文件时**:
1. 从内存（DashMap）中移除
2. 删除 `{file_id}.dat` 文件
3. 删除 `{file_id}.meta.json` 文件

**过期清理**:
- 自动清理超过 `max_age_seconds` 的文件
- 支持内存和磁盘同步清理

## 🚀 性能对比

| 指标 | Arc<RwLock<HashMap>> | DashMap |
|------|---------------------|---------|
| 读并发 | 多个读取共享锁 | 完全无锁读取 |
| 写并发 | 独占写锁，阻塞所有操作 | 分片锁，仅锁定部分数据 |
| API 复杂度 | 需要 `.read().await` / `.write().await` | 直接 `.get()` / `.insert()` |
| 内存开销 | 较低 | 略高（分片开销） |
| 适用场景 | 读多写少 | 高并发读写 |

## 📝 API 变更

### 存储文件
```rust
// 现在同时存储到内存和文件系统
pub async fn store(&self, filename: String, data: Vec<u8>) -> Result<String, AppError>
```

### 获取文件
```rust
// 优先从内存获取，支持过期检查
pub async fn retrieve(&self, file_id: &str) -> Result<(String, Vec<u8>), AppError>
```

### 删除文件
```rust
// 同时删除内存和文件系统中的文件
pub async fn delete(&self, file_id: &str) -> Result<(), AppError>
```

### 新增方法
```rust
// 从文件系统加载已存在的文件（启动时调用）
fn load_from_filesystem(&self) -> Result<(), AppError>

// 保存文件元数据到文件系统
fn save_metadata(&self, file_id: &str, filename: &str) -> Result<(), AppError>

// 获取文件路径
fn get_file_path(&self, file_id: &str) -> PathBuf

// 获取元数据路径
fn get_metadata_path(&self, file_id: &str) -> PathBuf
```

## 🧪 测试持久化功能

运行测试脚本验证持久化功能：

```powershell
# 运行持久化测试
.\examples\test_persistence.ps1
```

测试流程：
1. ✅ 启动服务器
2. ✅ 生成 Excel 文件并获取 file_id
3. ✅ 下载文件验证
4. ✅ 检查存储状态
5. ✅ 停止服务器
6. ✅ 验证文件系统中的 `.dat` 和 `.meta.json` 文件
7. ✅ 重新启动服务器
8. ✅ 验证文件自动加载
9. ✅ 下载重启前的文件，确认持久化成功

## 🔧 配置

在 `config/default.toml` 中配置：

```toml
[storage]
temp_dir = "./temp"           # 文件存储目录
max_age_seconds = 3600        # 文件最大保留时间（秒）
```

## ⚠️ 注意事项

1. **磁盘空间**: 文件会持久化到磁盘，需要足够的磁盘空间
2. **文件清理**: 过期文件会自动清理，但需要定期检查磁盘使用情况
3. **并发安全**: DashMap 保证并发安全，无需额外锁
4. **启动时间**: 如果 temp 目录有大量文件，启动时加载可能需要一些时间
5. **文件完整性**: 系统会验证 `.dat` 和 `.meta.json` 的匹配性

## 📊 依赖更新

在 `Cargo.toml` 中新增：

```toml
# 并发集合
dashmap = "6.1"
```

移除的依赖：
- 不再需要 `std::sync::Arc` 和 `tokio::sync::RwLock` 用于文件存储

## 🎯 优势总结

✅ **高性能**: DashMap 提供更好的并发性能  
✅ **持久化**: 文件重启后不会丢失  
✅ **自动加载**: 启动时自动从文件系统恢复  
✅ **过期清理**: 自动清理过期文件，节省空间  
✅ **简化代码**: 更简洁的 API，无需显式锁管理  
✅ **可靠性**: 内存和磁盘双重存储，更可靠

## 🔮 未来扩展

- [ ] 支持数据库持久化（PostgreSQL/Redis）
- [ ] 支持分布式文件存储（S3/OSS）
- [ ] 支持文件压缩以节省空间
- [ ] 支持文件访问统计和热点分析
- [ ] 支持文件版本管理
