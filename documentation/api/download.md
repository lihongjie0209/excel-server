# 下载接口

用于下载异步生成的 Excel 文件。支持 POST 和 GET 两种方式。

## GET 下载（推荐）

**方法**: GET  
**路径**: `/api/excel/download/{file_id}`

### 参数

| 参数 | 位置 | 类型 | 必填 | 说明 |
|------|------|------|------|------|
| `file_id` | Path | String | ✅ | 文件 ID (UUID) |

### 响应

**成功**: HTTP 200, 返回 Excel 文件流

**响应头**:
```
Content-Type: application/vnd.openxmlformats-officedocument.spreadsheetml.sheet
Content-Disposition: attachment; filename="output.xlsx"; filename*=UTF-8''%E6%96%87%E4%BB%B6.xlsx
Content-Length: 12345
```

**失败**: HTTP 200, 返回 JSON 错误

```json
{
  "code": 1003,
  "message": "文件不存在: invalid-file-id",
  "data": null,
  "success": false
}
```

### 示例

#### cURL

```bash
curl http://localhost:3000/api/excel/download/550e8400-e29b-41d4-a716-446655440000 \
  --output output.xlsx
```

#### 浏览器直接访问

```
http://localhost:3000/api/excel/download/550e8400-e29b-41d4-a716-446655440000
```

#### JavaScript

```javascript
// 方式 1: 浏览器重定向（最简单）
function downloadFile(fileId) {
  window.location.href = `/api/excel/download/${fileId}`;
}

// 方式 2: Fetch + Blob（可控制下载流程）
async function downloadFileWithProgress(fileId) {
  const response = await fetch(`/api/excel/download/${fileId}`);
  
  if (!response.ok) {
    const error = await response.json();
    console.error('下载失败:', error.message);
    return;
  }
  
  const blob = await response.blob();
  const url = window.URL.createObjectURL(blob);
  
  const a = document.createElement('a');
  a.href = url;
  a.download = `file_${fileId}.xlsx`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  window.URL.revokeObjectURL(url);
}

// 方式 3: 使用 <a> 标签
function createDownloadLink(fileId) {
  return `<a href="/api/excel/download/${fileId}" download>下载文件</a>`;
}
```

#### Python

```python
import requests

file_id = "550e8400-e29b-41d4-a716-446655440000"
url = f"http://localhost:3000/api/excel/download/{file_id}"

response = requests.get(url)

if response.ok:
    with open('output.xlsx', 'wb') as f:
        f.write(response.content)
    print("下载成功")
else:
    error = response.json()
    print(f"下载失败: {error['message']}")
```

---

## POST 下载（传统方式）

**方法**: POST  
**路径**: `/api/excel/download`  
**Content-Type**: `application/json`

### 请求体

```json
{
  "file_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### 响应

与 GET 方式相同。

### 示例

#### cURL

```bash
curl -X POST http://localhost:3000/api/excel/download \
  -H "Content-Type: application/json" \
  -d '{"file_id": "550e8400-e29b-41d4-a716-446655440000"}' \
  --output output.xlsx
```

#### JavaScript

```javascript
async function downloadFilePost(fileId) {
  const response = await fetch('/api/excel/download', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ file_id: fileId })
  });

  if (response.ok) {
    const blob = await response.blob();
    // ... 处理 blob
  } else {
    const error = await response.json();
    console.error('下载失败:', error.message);
  }
}
```

---

## 中文文件名支持

支持 RFC 5987 标准，完美显示中文文件名。

### 响应头示例

```
Content-Disposition: attachment; filename="report.xlsx"; filename*=UTF-8''%E6%8A%A5%E8%A1%A8.xlsx
```

- `filename="report.xlsx"` - ASCII 后备名称
- `filename*=UTF-8''%E6%8A%A5%E8%A1%A8.xlsx` - UTF-8 编码的中文名称

### 浏览器兼容性

| 浏览器 | 版本 | 支持 |
|--------|------|------|
| Chrome | 所有版本 | ✅ |
| Firefox | 所有版本 | ✅ |
| Safari | 所有版本 | ✅ |
| Edge | 所有版本 | ✅ |
| IE | 11+ | ⚠️ 部分支持 |

---

## 错误处理

### 文件不存在 (1003)

```json
{
  "code": 1003,
  "message": "文件不存在: invalid-file-id",
  "data": null,
  "success": false
}
```

**原因**:
- 文件 ID 无效
- 文件已过期（默认 1 小时）
- 文件已被删除

**解决方案**:
- 检查 file_id 格式是否正确
- 重新生成文件
- 增加文件 TTL 配置

### 参数错误 (1001)

```json
{
  "code": 1001,
  "message": "缺少必填字段: file_id",
  "data": null,
  "success": false
}
```

**原因**:
- POST 请求缺少 file_id 字段

### JavaScript 错误处理示例

```javascript
async function safeDownload(fileId) {
  try {
    const response = await fetch(`/api/excel/download/${fileId}`);
    
    // 检查响应类型
    const contentType = response.headers.get('content-type');
    
    if (contentType?.includes('spreadsheet')) {
      // 成功，是 Excel 文件
      const blob = await response.blob();
      return { success: true, blob };
    } else {
      // 失败，是 JSON 错误
      const error = await response.json();
      return { success: false, error };
    }
  } catch (err) {
    return { 
      success: false, 
      error: { message: '网络错误或服务器无响应' } 
    };
  }
}

// 使用
const result = await safeDownload(fileId);
if (result.success) {
  // 下载 blob
  downloadBlob(result.blob, 'report.xlsx');
} else {
  // 显示错误
  alert(result.error.message);
}
```

---

## GET vs POST 对比

| 特性 | GET | POST |
|------|-----|------|
| URL 长度 | 有限制 | 无限制 |
| 缓存 | 支持 | 不支持 |
| 浏览器直接访问 | ✅ | ❌ |
| 书签/分享 | ✅ | ❌ |
| 安全性 | URL 可见 | Body 不可见 |
| RESTful | ✅ | ❌ |

**推荐使用 GET 方式**，因为：
- 符合 RESTful 规范
- 可直接在浏览器打开
- 可分享链接
- 支持 CDN 缓存

---

## 存储状态查询

查看当前存储的文件数量。

**方法**: POST  
**路径**: `/api/excel/status`

### 响应

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

### 示例

```bash
curl -X POST http://localhost:3000/api/excel/status \
  -H "Content-Type: application/json"
```

```javascript
async function getStorageStatus() {
  const response = await fetch('/api/excel/status', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' }
  });
  
  const result = await response.json();
  console.log('当前文件数:', result.data.file_count);
}
```

---

## 高级用法

### 1. 断点续传

```javascript
async function downloadWithResume(fileId) {
  const response = await fetch(`/api/excel/download/${fileId}`, {
    headers: {
      'Range': 'bytes=0-1023'  // 下载前 1KB
    }
  });
  
  // 检查是否支持断点续传
  if (response.status === 206) {
    console.log('支持断点续传');
  }
}
```

### 2. 下载进度

```javascript
async function downloadWithProgress(fileId, onProgress) {
  const response = await fetch(`/api/excel/download/${fileId}`);
  const contentLength = response.headers.get('content-length');
  const total = parseInt(contentLength, 10);
  
  let loaded = 0;
  const reader = response.body.getReader();
  const chunks = [];
  
  while (true) {
    const { done, value } = await reader.read();
    if (done) break;
    
    chunks.push(value);
    loaded += value.length;
    onProgress({ loaded, total, percent: (loaded / total) * 100 });
  }
  
  return new Blob(chunks);
}

// 使用
downloadWithProgress(fileId, ({ percent }) => {
  console.log(`下载进度: ${percent.toFixed(2)}%`);
});
```

### 3. 批量下载

```javascript
async function downloadMultiple(fileIds) {
  const downloads = fileIds.map(id => 
    fetch(`/api/excel/download/${id}`).then(r => r.blob())
  );
  
  const blobs = await Promise.all(downloads);
  
  // 打包成 ZIP（需要 JSZip 库）
  const zip = new JSZip();
  blobs.forEach((blob, i) => {
    zip.file(`file_${i + 1}.xlsx`, blob);
  });
  
  const zipBlob = await zip.generateAsync({ type: 'blob' });
  downloadBlob(zipBlob, 'files.zip');
}
```

---

## 下一步

- [生成接口](/api/generate) - 了解如何生成文件
- [DSL 规范](/dsl/overview) - Excel 结构定义
- [快速开始](/guide/getting-started) - 完整示例
