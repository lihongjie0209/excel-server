# API 概览

Excel Server 提供完整的 RESTful API，支持 Excel 文件的生成、下载和管理。

## 基础信息

- **Base URL**: `http://localhost:3000`
- **Content-Type**: `application/json`
- **响应格式**: JSON

## API 端点列表

### Excel 生成

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/excel/generate` | 直接生成并返回 Excel 文件 |
| POST | `/api/excel/async` | 异步生成，返回 file_id |

### Excel 下载

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/excel/download` | 通过 file_id 下载（POST） |
| GET | `/api/excel/download/{file_id}` | 通过 file_id 下载（GET） |

### 系统管理

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/excel/status` | 查看存储状态 |
| GET | `/health` | 健康检查 |
| GET | `/metrics` | Prometheus 监控指标 |

### 文档

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/swagger-ui/` | Swagger UI 交互式文档 |
| GET | `/api-docs/openapi.json` | OpenAPI 3.0 规范文件 |

## 统一响应格式

所有业务接口返回统一的 JSON 格式：

```json
{
  "code": 0,
  "message": "success",
  "data": { ... },
  "success": true
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `code` | Integer | 业务状态码，0 表示成功 |
| `message` | String | 提示信息 |
| `data` | Any/null | 业务数据 |
| `success` | Boolean | 操作是否成功 |

### 业务状态码

| Code | 说明 | 示例 |
|------|------|------|
| 0 | 成功 | 操作成功完成 |
| 1001 | 参数错误 | 缺少必填字段 |
| 1003 | 资源不存在 | 文件 ID 不存在或已过期 |
| 2001 | Excel 生成失败 | DSL 格式错误 |
| 2002 | 存储错误 | 磁盘写入失败 |
| 5000 | 内部错误 | 服务器内部错误 |

## 成功响应示例

### 异步生成成功

```json
{
  "code": 0,
  "message": "success",
  "data": {
    "file_id": "550e8400-e29b-41d4-a716-446655440000"
  },
  "success": true
}
```

### 存储状态查询

```json
{
  "code": 0,
  "message": "success",
  "data": {
    "file_count": 3
  },
  "success": true
}
```

## 错误响应示例

### 文件不存在

```json
{
  "code": 1003,
  "message": "文件不存在: invalid-file-id",
  "data": null,
  "success": false
}
```

HTTP Status: `200 OK` (注意：业务错误仍返回 200)

### DSL 格式错误

```json
{
  "code": 2001,
  "message": "Excel 生成失败: missing required field `sheets`",
  "data": null,
  "success": false
}
```

## 认证和授权

当前版本不包含认证机制。如需在生产环境使用，建议：

1. 在前置网关（Nginx/Traefik）添加认证
2. 使用 API 密钥验证
3. 集成 OAuth2/JWT

## 速率限制

当前版本不包含速率限制。建议在生产环境：

1. 使用反向代理（Nginx）限制请求速率
2. 使用 Redis 实现分布式限流
3. 根据 IP 或用户进行限流

## CORS 配置

服务默认允许所有来源的跨域请求：

```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, OPTIONS
Access-Control-Allow-Headers: Content-Type
```

生产环境建议配置具体的允许来源。

## 文件大小限制

- **请求体大小**: 默认 10MB
- **生成的 Excel 文件**: 无限制（受磁盘空间限制）

## 超时设置

- **请求超时**: 30 秒
- **文件生成超时**: 根据文件大小自动调整
- **文件过期时间**: 默认 3600 秒（1 小时）

## 最佳实践

### 1. 使用异步模式处理大文件

```javascript
// 推荐：异步生成
const { data } = await fetch('/api/excel/async', {...}).then(r => r.json());
const fileId = data.file_id;

// 延迟下载或分享链接
window.location.href = `/api/excel/download/${fileId}`;
```

### 2. 处理错误

```javascript
const response = await fetch('/api/excel/async', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify(dsl)
});

const result = await response.json();

if (result.success) {
  console.log('文件ID:', result.data.file_id);
} else {
  console.error('错误:', result.message);
  // 根据 code 进行不同处理
  switch (result.code) {
    case 1001:
      alert('请检查输入参数');
      break;
    case 2001:
      alert('Excel 格式错误');
      break;
    default:
      alert('服务器错误，请稍后重试');
  }
}
```

### 3. 文件过期处理

```javascript
async function downloadWithRetry(fileId, maxRetries = 3) {
  for (let i = 0; i < maxRetries; i++) {
    const response = await fetch(`/api/excel/download/${fileId}`);
    
    if (response.ok) {
      return response.blob();
    }
    
    const error = await response.json();
    if (error.code === 1003) {
      console.error('文件已过期或不存在');
      return null;
    }
    
    // 其他错误，重试
    await new Promise(r => setTimeout(r, 1000 * (i + 1)));
  }
  
  throw new Error('下载失败');
}
```

## 下一步

- [生成接口](/api/generate) - 详细的生成接口说明
- [下载接口](/api/download) - 详细的下载接口说明
- [客户端示例](/api/clients/javascript) - 各种语言的客户端示例

## 在线测试

访问 [Swagger UI](http://localhost:3000/swagger-ui/) 在线测试所有 API。
