# 生成接口

Excel Server 提供两种生成模式：同步直接返回文件、异步返回文件 ID。

## 同步生成 (POST /api/excel/generate)

直接生成并返回 Excel 文件，适合小文件和实时场景。

### 请求

**方法**: POST  
**路径**: `/api/excel/generate`  
**Content-Type**: `application/json`

**请求体**: [DSL JSON](/dsl/overview)

### 响应

**成功**: HTTP 200, 直接返回 Excel 文件流

**响应头**:
```
Content-Type: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet
Content-Disposition: attachment; filename="output.xlsx"; filename*=UTF-8''output.xlsx
```

**失败**: HTTP 200, 返回 JSON 错误信息

```json
{
  "code": 2001,
  "message": "Excel 生成失败: missing field `sheets`",
  "data": null,
  "success": false
}
```

### 示例

#### cURL

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
  --output output.xlsx
```

#### JavaScript

```javascript
async function generateExcel(dsl) {
  const response = await fetch('http://localhost:3000/api/excel/generate', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(dsl)
  });

  if (response.ok && response.headers.get('content-type')?.includes('spreadsheet')) {
    const blob = await response.blob();
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'output.xlsx';
    a.click();
    window.URL.revokeObjectURL(url);
  } else {
    const error = await response.json();
    console.error('生成失败:', error.message);
  }
}
```

#### Python

```python
import requests

dsl = {
    "sheets": [{
        "name": "Sheet1",
        "cells": [
            {"row": 0, "col": 0, "value": "Hello", "value_type": "string"}
        ]
    }]
}

response = requests.post(
    'http://localhost:3000/api/excel/generate',
    json=dsl
)

if response.ok:
    with open('output.xlsx', 'wb') as f:
        f.write(response.content)
else:
    error = response.json()
    print(f"Error: {error['message']}")
```

### 使用场景

✅ **适合**:
- 文件小于 1MB
- 需要立即下载
- 单用户实时生成
- 简单数据导出

❌ **不适合**:
- 大文件（> 10MB）
- 高并发场景
- 需要分享链接
- 生成时间长（> 5秒）

---

## 异步生成 (POST /api/excel/async)

异步生成，返回文件 ID，适合大文件和高并发场景。

### 请求

**方法**: POST  
**路径**: `/api/excel/async`  
**Content-Type**: `application/json`

**请求体**: [DSL JSON](/dsl/overview)

### 响应

**成功**: HTTP 200, 返回文件 ID

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

**失败**: HTTP 200, 返回错误信息

```json
{
  "code": 2001,
  "message": "Excel 生成失败: invalid cell range",
  "data": null,
  "success": false
}
```

### 示例

#### cURL

```bash
# 1. 生成文件
FILE_ID=$(curl -X POST http://localhost:3000/api/excel/async \
  -H "Content-Type: application/json" \
  -d '{
    "sheets": [{
      "name": "Sheet1",
      "cells": [
        {"row": 0, "col": 0, "value": "Hello", "value_type": "string"}
      ]
    }]
  }' | jq -r '.data.file_id')

# 2. 下载文件
curl http://localhost:3000/api/excel/download/${FILE_ID} --output output.xlsx
```

#### JavaScript

```javascript
async function generateAndDownload(dsl) {
  // 1. 异步生成
  const generateResp = await fetch('http://localhost:3000/api/excel/async', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dsl)
  });

  const result = await generateResp.json();
  
  if (!result.success) {
    console.error('生成失败:', result.message);
    return;
  }

  const fileId = result.data.file_id;
  console.log('文件ID:', fileId);

  // 2. 下载文件
  window.location.href = `/api/excel/download/${fileId}`;
  
  // 或者使用 fetch
  const downloadResp = await fetch(`/api/excel/download/${fileId}`);
  const blob = await downloadResp.blob();
  // ... 处理 blob
}
```

#### Python

```python
import requests
import time

# 1. 异步生成
dsl = {
    "sheets": [{
        "name": "Sheet1",
        "cells": [
            {"row": 0, "col": 0, "value": "Hello", "value_type": "string"}
        ]
    }]
}

response = requests.post('http://localhost:3000/api/excel/async', json=dsl)
result = response.json()

if not result['success']:
    print(f"生成失败: {result['message']}")
    exit(1)

file_id = result['data']['file_id']
print(f"文件ID: {file_id}")

# 2. 下载文件
download_url = f'http://localhost:3000/api/excel/download/{file_id}'
response = requests.get(download_url)

if response.ok:
    with open('output.xlsx', 'wb') as f:
        f.write(response.content)
    print("下载成功")
else:
    error = response.json()
    print(f"下载失败: {error['message']}")
```

### 使用场景

✅ **适合**:
- 大文件（> 1MB）
- 高并发场景
- 需要分享链接
- 生成时间长
- 延迟下载

✅ **优点**:
- 非阻塞，快速响应
- 可分享文件 ID
- 支持断点续传
- 减轻服务器压力

---

## 性能对比

| 指标 | 同步生成 | 异步生成 |
|------|---------|---------|
| 响应时间 | 生成时间 + 传输时间 | < 10ms |
| 内存占用 | 文件大小 | 文件 ID |
| 并发能力 | 低 | 高 |
| 适用场景 | 小文件 | 大文件 |

## 文件过期

异步生成的文件默认保存 **1 小时**（3600 秒），过期后自动清理。

可通过配置文件修改:

```toml
# config/default.toml
[file_storage]
ttl = 7200  # 2 小时
```

## 下一步

- [下载接口](/api/download) - 如何下载生成的文件
- [DSL 规范](/dsl/overview) - 了解请求体格式
- [客户端示例](/api/clients/javascript) - 更多语言示例
