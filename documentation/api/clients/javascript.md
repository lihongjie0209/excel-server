# JavaScript 客户端

## 安装

### CDN 引入

```html
<script src="https://unpkg.com/axios/dist/axios.min.js"></script>
```

### npm 安装

```bash
npm install axios
```

## 基础用法

### 1. 同步生成并下载

```javascript
async function generateAndDownload(dsl) {
  const response = await fetch('http://localhost:3000/api/excel/generate', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(dsl)
  });

  if (response.ok) {
    const blob = await response.blob();
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'output.xlsx';
    a.click();
    window.URL.revokeObjectURL(url);
  }
}
```

### 2. 异步生成 + 下载

```javascript
async function asyncGenerateAndDownload(dsl) {
  // 1. 生成文件
  const generateResp = await fetch('http://localhost:3000/api/excel/async', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dsl)
  });

  const result = await generateResp.json();
  
  if (!result.success) {
    throw new Error(result.message);
  }

  const fileId = result.data.file_id;

  // 2. 下载文件（简单方式）
  window.location.href = `/api/excel/download/${fileId}`;
}
```

## React 示例

### 完整组件

```jsx
import React, { useState } from 'react';
import axios from 'axios';

function ExcelGenerator() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const generateExcel = async () => {
    setLoading(true);
    setError(null);

    const dsl = {
      sheets: [{
        name: "Sheet1",
        cells: [
          { row: 0, col: 0, value: "姓名", value_type: "string" },
          { row: 0, col: 1, value: "年龄", value_type: "string" },
          { row: 1, col: 0, value: "张三", value_type: "string" },
          { row: 1, col: 1, value: 25, value_type: "number" }
        ]
      }]
    };

    try {
      // 异步生成
      const { data } = await axios.post('/api/excel/async', dsl);
      
      if (!data.success) {
        throw new Error(data.message);
      }

      // 下载
      window.location.href = `/api/excel/download/${data.data.file_id}`;
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <button onClick={generateExcel} disabled={loading}>
        {loading ? '生成中...' : '生成 Excel'}
      </button>
      {error && <div style={{color: 'red'}}>{error}</div>}
    </div>
  );
}

export default ExcelGenerator;
```

### 使用 Hooks

```jsx
import { useState, useCallback } from 'react';

function useExcelGenerator(baseUrl = 'http://localhost:3000') {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const generate = useCallback(async (dsl) => {
    setLoading(true);
    setError(null);

    try {
      const response = await fetch(`${baseUrl}/api/excel/async`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(dsl)
      });

      const result = await response.json();

      if (!result.success) {
        throw new Error(result.message);
      }

      return result.data.file_id;
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [baseUrl]);

  const download = useCallback((fileId) => {
    window.location.href = `${baseUrl}/api/excel/download/${fileId}`;
  }, [baseUrl]);

  return { generate, download, loading, error };
}

// 使用
function App() {
  const { generate, download, loading, error } = useExcelGenerator();

  const handleGenerate = async () => {
    const dsl = { /* ... */ };
    const fileId = await generate(dsl);
    download(fileId);
  };

  return (
    <button onClick={handleGenerate} disabled={loading}>
      {loading ? '生成中...' : '生成 Excel'}
    </button>
  );
}
```

## Vue 示例

### Vue 3 Composition API

```vue
<template>
  <div>
    <button @click="generateExcel" :disabled="loading">
      {{ loading ? '生成中...' : '生成 Excel' }}
    </button>
    <div v-if="error" class="error">{{ error }}</div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import axios from 'axios';

const loading = ref(false);
const error = ref(null);

const generateExcel = async () => {
  loading.value = true;
  error.value = null;

  const dsl = {
    sheets: [{
      name: "Sheet1",
      cells: [
        { row: 0, col: 0, value: "Hello", value_type: "string" }
      ]
    }]
  };

  try {
    const { data } = await axios.post('/api/excel/async', dsl);
    
    if (!data.success) {
      throw new Error(data.message);
    }

    window.location.href = `/api/excel/download/${data.data.file_id}`;
  } catch (err) {
    error.value = err.message;
  } finally {
    loading.value = false;
  }
};
</script>

<style>
.error {
  color: red;
  margin-top: 10px;
}
</style>
```

### Vue 2

```vue
<template>
  <div>
    <button @click="generateExcel" :disabled="loading">
      {{ loading ? '生成中...' : '生成 Excel' }}
    </button>
    <div v-if="error" class="error">{{ error }}</div>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      loading: false,
      error: null
    };
  },
  methods: {
    async generateExcel() {
      this.loading = true;
      this.error = null;

      const dsl = {
        sheets: [{
          name: "Sheet1",
          cells: [
            { row: 0, col: 0, value: "Hello", value_type: "string" }
          ]
        }]
      };

      try {
        const { data } = await axios.post('/api/excel/async', dsl);
        
        if (!data.success) {
          throw new Error(data.message);
        }

        window.location.href = `/api/excel/download/${data.data.file_id}`;
      } catch (err) {
        this.error = err.message;
      } finally {
        this.loading = false;
      }
    }
  }
};
</script>
```

## TypeScript 类型定义

```typescript
// types.ts
export interface Cell {
  row: number;
  col: number;
  value: string | number | boolean;
  value_type: 'string' | 'number' | 'boolean' | 'formula';
  style?: CellStyle;
}

export interface CellStyle {
  font_size?: number;
  font_color?: string;
  bg_color?: string;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
}

export interface Sheet {
  name: string;
  cells: Cell[];
  column_widths?: Record<number, number>;
  row_heights?: Record<number, number>;
}

export interface ExcelDSL {
  sheets: Sheet[];
  document_properties?: {
    title?: string;
    author?: string;
    subject?: string;
    company?: string;
  };
}

export interface ApiResponse<T> {
  code: number;
  message: string;
  data: T | null;
  success: boolean;
}

export interface AsyncGenerateResponse {
  file_id: string;
}
```

```typescript
// api.ts
import type { ExcelDSL, ApiResponse, AsyncGenerateResponse } from './types';

const BASE_URL = 'http://localhost:3000';

export async function asyncGenerate(dsl: ExcelDSL): Promise<string> {
  const response = await fetch(`${BASE_URL}/api/excel/async`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dsl)
  });

  const result: ApiResponse<AsyncGenerateResponse> = await response.json();

  if (!result.success) {
    throw new Error(result.message);
  }

  return result.data!.file_id;
}

export function downloadFile(fileId: string): void {
  window.location.href = `${BASE_URL}/api/excel/download/${fileId}`;
}

export async function generateExcel(dsl: ExcelDSL): Promise<Blob> {
  const response = await fetch(`${BASE_URL}/api/excel/generate`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dsl)
  });

  if (!response.ok) {
    const error: ApiResponse<null> = await response.json();
    throw new Error(error.message);
  }

  return response.blob();
}
```

```typescript
// usage.ts
import { asyncGenerate, downloadFile, generateExcel } from './api';
import type { ExcelDSL } from './types';

const dsl: ExcelDSL = {
  sheets: [{
    name: "Sheet1",
    cells: [
      { row: 0, col: 0, value: "Hello", value_type: "string" }
    ]
  }]
};

// 异步方式
const fileId = await asyncGenerate(dsl);
downloadFile(fileId);

// 同步方式
const blob = await generateExcel(dsl);
```

## 下一步

- [API 概览](/api/overview) - 所有可用接口
- [DSL 规范](/dsl/overview) - DSL 完整说明
- [快速开始](/guide/getting-started) - 快速上手
