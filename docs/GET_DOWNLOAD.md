# GET 下载接口说明

## 📋 新增功能

添加了 **GET 方法**的文件下载接口，方便前端直接通过 URL 下载文件。

## 🆚 两种下载方式对比

### POST 方法（原有）
```http
POST /api/excel/download
Content-Type: application/json

{
  "file_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**特点**:
- ✅ 支持请求体传参
- ✅ 适合需要鉴权的场景（可在 body 中传递 token）
- ✅ 支持复杂参数
- ❌ 前端使用需要 fetch/axios
- ❌ 不能直接在浏览器地址栏访问
- ❌ 不能用 `<a>` 标签直接下载

### GET 方法（新增）✨
```http
GET /api/excel/download/{file_id}
```

**特点**:
- ✅ URL 直接访问，更简单
- ✅ 可用 `<a>` 标签直接下载
- ✅ 可在浏览器地址栏直接访问
- ✅ 支持 `window.location.href` 跳转下载
- ✅ 更符合 RESTful 规范
- ❌ file_id 暴露在 URL 中
- ❌ 不适合传递复杂参数

## 🚀 前端使用示例

### 1. HTML 直接下载

```html
<a href="http://localhost:3000/api/excel/download/550e8400-e29b-41d4-a716-446655440000" 
   download="report.xlsx">
  点击下载 Excel
</a>

<!-- 或使用按钮 -->
<button onclick="downloadExcel()">下载报表</button>

<script>
function downloadExcel() {
  const fileId = '550e8400-e29b-41d4-a716-446655440000';
  window.location.href = `http://localhost:3000/api/excel/download/${fileId}`;
}
</script>
```

### 2. JavaScript/TypeScript

```javascript
// 直接跳转下载
const fileId = '550e8400-e29b-41d4-a716-446655440000';
window.location.href = `http://localhost:3000/api/excel/download/${fileId}`;

// 使用 fetch 下载（可监控进度）
async function downloadExcel(fileId) {
  try {
    const response = await fetch(`http://localhost:3000/api/excel/download/${fileId}`);
    
    if (!response.ok) {
      throw new Error('下载失败');
    }
    
    const blob = await response.blob();
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'report.xlsx';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  } catch (error) {
    console.error('下载失败:', error);
  }
}
```

### 3. React 示例

```tsx
import React from 'react';

interface DownloadButtonProps {
  fileId: string;
  fileName?: string;
}

// 方式 1: 直接跳转
const DownloadButton1: React.FC<DownloadButtonProps> = ({ fileId }) => {
  const handleDownload = () => {
    window.location.href = `http://localhost:3000/api/excel/download/${fileId}`;
  };
  
  return <button onClick={handleDownload}>下载 Excel</button>;
};

// 方式 2: fetch 下载
const DownloadButton2: React.FC<DownloadButtonProps> = ({ fileId, fileName = 'report.xlsx' }) => {
  const [loading, setLoading] = React.useState(false);
  
  const handleDownload = async () => {
    setLoading(true);
    try {
      const response = await fetch(`http://localhost:3000/api/excel/download/${fileId}`);
      const blob = await response.blob();
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = fileName;
      a.click();
      URL.revokeObjectURL(url);
    } catch (error) {
      console.error('下载失败:', error);
    } finally {
      setLoading(false);
    }
  };
  
  return (
    <button onClick={handleDownload} disabled={loading}>
      {loading ? '下载中...' : '下载 Excel'}
    </button>
  );
};

// 方式 3: 使用 a 标签
const DownloadLink: React.FC<DownloadButtonProps> = ({ fileId, fileName = 'report.xlsx' }) => {
  return (
    <a 
      href={`http://localhost:3000/api/excel/download/${fileId}`}
      download={fileName}
      className="download-link"
    >
      下载 Excel
    </a>
  );
};
```

### 4. Vue 示例

```vue
<template>
  <div>
    <!-- 方式 1: 直接跳转 -->
    <button @click="downloadDirect">下载 Excel（直接）</button>
    
    <!-- 方式 2: fetch 下载 -->
    <button @click="downloadWithFetch" :disabled="loading">
      {{ loading ? '下载中...' : '下载 Excel（Fetch）' }}
    </button>
    
    <!-- 方式 3: 使用 a 标签 -->
    <a :href="downloadUrl" download="report.xlsx">下载 Excel（链接）</a>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

const fileId = ref('550e8400-e29b-41d4-a716-446655440000');
const loading = ref(false);

const downloadUrl = computed(() => 
  `http://localhost:3000/api/excel/download/${fileId.value}`
);

const downloadDirect = () => {
  window.location.href = downloadUrl.value;
};

const downloadWithFetch = async () => {
  loading.value = true;
  try {
    const response = await fetch(downloadUrl.value);
    const blob = await response.blob();
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'report.xlsx';
    a.click();
    URL.revokeObjectURL(url);
  } catch (error) {
    console.error('下载失败:', error);
  } finally {
    loading.value = false;
  }
};
</script>
```

### 5. axios 示例

```javascript
import axios from 'axios';

async function downloadExcel(fileId) {
  try {
    const response = await axios.get(
      `http://localhost:3000/api/excel/download/${fileId}`,
      {
        responseType: 'blob', // 重要：指定响应类型为 blob
      }
    );
    
    // 创建下载链接
    const url = URL.createObjectURL(response.data);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'report.xlsx';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  } catch (error) {
    console.error('下载失败:', error);
  }
}

// 或使用 axios 拦截器统一处理
axios.interceptors.response.use(
  response => {
    // 检查是否是文件下载响应
    if (response.headers['content-type']?.includes('spreadsheetml')) {
      const url = URL.createObjectURL(response.data);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'report.xlsx';
      a.click();
      URL.revokeObjectURL(url);
    }
    return response;
  },
  error => Promise.reject(error)
);
```

## 🧪 测试

运行测试脚本：

```powershell
# 启动服务器
cargo run

# 在另一个终端运行测试
.\examples\test_get_download.ps1
```

## 📊 API 文档

访问 Swagger UI 查看完整 API 文档：
```
http://localhost:3000/swagger-ui/
```

新接口路径：
- `GET /api/excel/download/{file_id}` - 通过 file_id 下载 Excel 文件

## 💡 使用建议

| 场景 | 推荐方法 |
|------|---------|
| 简单下载 | GET（更方便） |
| 需要鉴权 | POST（可在 body 传 token） |
| 前端直接下载 | GET（`<a>` 标签或 `window.location.href`） |
| 需要进度监控 | GET + fetch（可监听 progress） |
| 分享下载链接 | GET（URL 直接访问） |
| 需要复杂参数 | POST（支持 JSON body） |

## 🔒 安全建议

1. **file_id 防猜测**: 使用 UUID v4，难以猜测
2. **访问控制**: 可在 file_id 中加入签名或时间戳
3. **速率限制**: 对下载接口添加速率限制
4. **CORS 配置**: 根据需求配置合适的 CORS 策略

## 🎯 完整工作流程

```javascript
// 1. 异步生成 Excel
const response = await fetch('http://localhost:3000/api/excel/async', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    // DSL 数据
    properties: { title: 'Sales Report' },
    sheets: [{ name: 'Sheet1', cells: [...] }]
  })
});

const { data } = await response.json();
const fileId = data.file_id;

// 2. 使用 GET 方法下载（推荐）
window.location.href = `http://localhost:3000/api/excel/download/${fileId}`;

// 或使用 POST 方法下载
fetch('http://localhost:3000/api/excel/download', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ file_id: fileId })
})
.then(res => res.blob())
.then(blob => {
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'report.xlsx';
  a.click();
  URL.revokeObjectURL(url);
});
```

## 📝 注意事项

1. **文件过期**: 文件默认保留 1 小时（可配置 `max_age_seconds`）
2. **文件不存在**: 返回 JSON 错误响应（`code: 1003`）
3. **Content-Disposition**: 自动设置文件名，支持下载弹窗
4. **Content-Type**: `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`

---

**版本**: v0.2.0+  
**更新日期**: 2026-01-01
