# GET 下载接口

从 v0.2.0 开始，支持 GET 方法下载文件，提供更好的用户体验。

## 为什么需要 GET 下载？

### POST 下载的局限

```javascript
// POST 方式：需要额外代码
const response = await fetch('/api/excel/download', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ file_id: 'xxx' })
});
const blob = await response.blob();
// ... 处理 blob
```

### GET 下载的优势

```javascript
// GET 方式：简单直接
window.location.href = `/api/excel/download/${fileId}`;
```

✅ **优点**:
- 浏览器直接下载，无需 JavaScript
- 可分享链接
- 支持书签
- 符合 RESTful 规范
- 支持 CDN 缓存

## 使用方式

### 1. 浏览器直接访问

```
http://localhost:3000/api/excel/download/550e8400-e29b-41d4-a716-446655440000
```

浏览器自动开始下载文件。

### 2. HTML 链接

```html
<a href="/api/excel/download/550e8400-e29b-41d4-a716-446655440000" download>
  下载报表
</a>
```

### 3. JavaScript 重定向

```javascript
const fileId = result.data.file_id;
window.location.href = `/api/excel/download/${fileId}`;
```

### 4. Fetch API

```javascript
const response = await fetch(`/api/excel/download/${fileId}`);
const blob = await response.blob();
```

## 对比两种方式

### GET vs POST

| 特性 | GET | POST |
|------|-----|------|
| **URL 格式** | `/api/excel/download/{id}` | `/api/excel/download` |
| **参数位置** | Path | Body |
| **浏览器支持** | 原生支持 | 需要 JavaScript |
| **链接分享** | ✅ 支持 | ❌ 不支持 |
| **RESTful** | ✅ 符合 | ❌ 不符合 |
| **CDN 缓存** | ✅ 支持 | ❌ 不支持 |
| **实现复杂度** | 简单 | 复杂 |

### 推荐使用场景

**使用 GET**:
- ✅ 给用户分享下载链接
- ✅ 在 HTML 中创建下载按钮
- ✅ 浏览器直接下载
- ✅ 邮件中发送下载链接

**使用 POST**:
- ⚠️ 需要额外安全验证
- ⚠️ 文件 ID 不能暴露
- ⚠️ 特殊权限控制

## 示例代码

### React 组件

```jsx
function DownloadButton({ fileId }) {
  return (
    <a 
      href={`/api/excel/download/${fileId}`}
      className="btn btn-primary"
      download
    >
      下载 Excel
    </a>
  );
}
```

### Vue 组件

```vue
<template>
  <a :href="`/api/excel/download/${fileId}`" class="btn" download>
    下载 Excel
  </a>
</template>

<script setup>
defineProps({
  fileId: String
});
</script>
```

### 分享链接

```javascript
function shareDownloadLink(fileId) {
  const url = `${window.location.origin}/api/excel/download/${fileId}`;
  
  // 复制到剪贴板
  navigator.clipboard.writeText(url).then(() => {
    alert('链接已复制: ' + url);
  });
}
```

### 邮件通知

```javascript
async function sendDownloadEmail(email, fileId) {
  const downloadUrl = `https://excel-server.com/api/excel/download/${fileId}`;
  
  await sendEmail({
    to: email,
    subject: '您的报表已生成',
    body: `点击下载: ${downloadUrl}`
  });
}
```

## 完整工作流

### 生成 → 分享链接

```javascript
async function generateAndShare(dsl) {
  // 1. 异步生成
  const response = await fetch('/api/excel/async', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dsl)
  });

  const result = await response.json();
  
  if (!result.success) {
    throw new Error(result.message);
  }

  // 2. 获取下载链接
  const fileId = result.data.file_id;
  const downloadUrl = `${location.origin}/api/excel/download/${fileId}`;

  // 3. 分享链接
  return downloadUrl;
}

// 使用
const url = await generateAndShare(myDSL);
console.log('分享此链接:', url);
```

## 安全考虑

### 文件 ID 的安全性

- ✅ 使用 UUID v4（随机性强）
- ✅ 文件自动过期（默认 1 小时）
- ✅ 无法枚举（猜测其他文件 ID）

### 防止滥用

```toml
# config/default.toml
[file_storage]
ttl = 1800  # 30 分钟过期
```

### 添加访问控制（可选）

如需额外安全，可在反向代理层添加：

```nginx
# Nginx 配置
location /api/excel/download/ {
    # 要求认证
    auth_request /auth;
    proxy_pass http://excel_server;
}
```

## 错误处理

### 文件不存在

```javascript
async function safeDownload(fileId) {
  const response = await fetch(`/api/excel/download/${fileId}`);
  
  if (response.headers.get('content-type')?.includes('json')) {
    // 返回的是错误 JSON
    const error = await response.json();
    alert(`下载失败: ${error.message}`);
    return false;
  }
  
  // 成功，处理文件
  const blob = await response.blob();
  return true;
}
```

## 迁移指南

### 从 POST 迁移到 GET

**之前 (POST)**:
```javascript
const response = await fetch('/api/excel/download', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ file_id: fileId })
});
```

**现在 (GET)**:
```javascript
window.location.href = `/api/excel/download/${fileId}`;
```

### 兼容性

POST 方式仍然支持，不会移除。可根据场景选择：

- 新代码：推荐 GET
- 旧代码：继续使用 POST（无需修改）

## 监控和日志

### 访问日志

```log
[INFO] GET /api/excel/download/550e8400-e29b-41d4-a716-446655440000
[INFO] File downloaded: 550e8400-e29b-41d4-a716-446655440000 (12.3 KB)
```

### Prometheus 指标

```
# 下载次数
excel_server_downloads_total{method="get"} 1234

# 下载失败
excel_server_download_errors_total{code="1003"} 56
```

## 下一步

- [下载接口](/api/download) - 完整 API 文档
- [文件持久化](/guide/persistence) - 了解文件存储
- [快速开始](/guide/getting-started) - 开始使用
