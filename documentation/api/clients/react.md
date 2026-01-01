# React 客户端

完整的 React 集成示例。

## 安装依赖

```bash
npm install axios
# 或
npm install @tanstack/react-query axios
```

## 基础组件

### 简单导出按钮

```jsx
import React, { useState } from 'react';

function ExportButton() {
  const [loading, setLoading] = useState(false);

  const handleExport = async () => {
    setLoading(true);

    const dsl = {
      sheets: [{
        name: "Sheet1",
        cells: [
          { row: 0, col: 0, value: "Hello", value_type: "string" }
        ]
      }]
    };

    try {
      const response = await fetch('/api/excel/async', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(dsl)
      });

      const result = await response.json();
      
      if (result.success) {
        window.location.href = `/api/excel/download/${result.data.file_id}`;
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <button onClick={handleExport} disabled={loading}>
      {loading ? '导出中...' : '导出 Excel'}
    </button>
  );
}
```

## 自定义 Hook

### useExcelExport Hook

```jsx
import { useState, useCallback } from 'react';

export function useExcelExport(baseUrl = '/api/excel') {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [fileId, setFileId] = useState(null);

  const generate = useCallback(async (dsl) => {
    setLoading(true);
    setError(null);
    setFileId(null);

    try {
      const response = await fetch(`${baseUrl}/async`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(dsl)
      });

      const result = await response.json();

      if (!result.success) {
        throw new Error(result.message);
      }

      const id = result.data.file_id;
      setFileId(id);
      return id;
    } catch (err) {
      setError(err.message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, [baseUrl]);

  const download = useCallback((id) => {
    window.location.href = `${baseUrl}/download/${id || fileId}`;
  }, [baseUrl, fileId]);

  const generateAndDownload = useCallback(async (dsl) => {
    const id = await generate(dsl);
    download(id);
  }, [generate, download]);

  return {
    generate,
    download,
    generateAndDownload,
    loading,
    error,
    fileId
  };
}
```

**使用**:

```jsx
function App() {
  const { generateAndDownload, loading, error } = useExcelExport();

  const handleExport = () => {
    generateAndDownload({
      sheets: [{
        name: "数据",
        cells: [...]
      }]
    });
  };

  return (
    <div>
      <button onClick={handleExport} disabled={loading}>
        {loading ? '导出中...' : '导出'}
      </button>
      {error && <div className="error">{error}</div>}
    </div>
  );
}
```

## React Query 集成

```jsx
import { useMutation } from '@tanstack/react-query';

async function generateExcel(dsl) {
  const response = await fetch('/api/excel/async', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dsl)
  });

  const result = await response.json();
  
  if (!result.success) {
    throw new Error(result.message);
  }

  return result.data.file_id;
}

function ExportComponent() {
  const mutation = useMutation({
    mutationFn: generateExcel,
    onSuccess: (fileId) => {
      window.location.href = `/api/excel/download/${fileId}`;
    }
  });

  const handleExport = () => {
    mutation.mutate({
      sheets: [{
        name: "数据",
        cells: [...]
      }]
    });
  };

  return (
    <button 
      onClick={handleExport} 
      disabled={mutation.isPending}
    >
      {mutation.isPending ? '导出中...' : '导出 Excel'}
      {mutation.isError && <div>{mutation.error.message}</div>}
    </button>
  );
}
```

## 完整示例

### 数据表导出

```jsx
import React from 'react';
import { useExcelExport } from './useExcelExport';

function UserTable({ users }) {
  const { generateAndDownload, loading } = useExcelExport();

  const exportUsers = () => {
    const dsl = {
      sheets: [{
        name: "用户列表",
        cells: [
          // 表头
          { row: 0, col: 0, value: "ID", value_type: "string", 
            style: { bold: true, bg_color: "#4472C4", font_color: "#FFFFFF" } },
          { row: 0, col: 1, value: "姓名", value_type: "string",
            style: { bold: true, bg_color: "#4472C4", font_color: "#FFFFFF" } },
          { row: 0, col: 2, value: "邮箱", value_type: "string",
            style: { bold: true, bg_color: "#4472C4", font_color: "#FFFFFF" } },
          
          // 数据行
          ...users.flatMap((user, index) => [
            { row: index + 1, col: 0, value: user.id, value_type: "number" },
            { row: index + 1, col: 1, value: user.name, value_type: "string" },
            { row: index + 1, col: 2, value: user.email, value_type: "string" }
          ])
        ],
        column_widths: {
          0: 10,
          1: 20,
          2: 30
        }
      }]
    };

    generateAndDownload(dsl);
  };

  return (
    <div>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>姓名</th>
            <th>邮箱</th>
          </tr>
        </thead>
        <tbody>
          {users.map(user => (
            <tr key={user.id}>
              <td>{user.id}</td>
              <td>{user.name}</td>
              <td>{user.email}</td>
            </tr>
          ))}
        </tbody>
      </table>
      
      <button onClick={exportUsers} disabled={loading}>
        {loading ? '导出中...' : '导出 Excel'}
      </button>
    </div>
  );
}
```

## TypeScript 支持

```tsx
import { useState, useCallback } from 'react';

interface Cell {
  row: number;
  col: number;
  value: string | number | boolean;
  value_type: 'string' | 'number' | 'boolean' | 'formula';
  style?: CellStyle;
}

interface CellStyle {
  bold?: boolean;
  italic?: boolean;
  font_size?: number;
  font_color?: string;
  bg_color?: string;
}

interface Sheet {
  name: string;
  cells: Cell[];
  column_widths?: Record<number, number>;
}

interface ExcelDSL {
  sheets: Sheet[];
  document_properties?: {
    title?: string;
    author?: string;
  };
}

interface ApiResponse<T> {
  code: number;
  message: string;
  data: T | null;
  success: boolean;
}

interface GenerateResponse {
  file_id: string;
}

export function useExcelExport() {
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const generate = useCallback(async (dsl: ExcelDSL): Promise<string> => {
    setLoading(true);
    setError(null);

    try {
      const response = await fetch('/api/excel/async', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(dsl)
      });

      const result: ApiResponse<GenerateResponse> = await response.json();

      if (!result.success || !result.data) {
        throw new Error(result.message);
      }

      return result.data.file_id;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Unknown error';
      setError(message);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  const download = useCallback((fileId: string): void => {
    window.location.href = `/api/excel/download/${fileId}`;
  }, []);

  const generateAndDownload = useCallback(async (dsl: ExcelDSL): Promise<void> => {
    const fileId = await generate(dsl);
    download(fileId);
  }, [generate, download]);

  return {
    generate,
    download,
    generateAndDownload,
    loading,
    error
  };
}
```

## 下一步

- [JavaScript 客户端](/api/clients/javascript) - Vanilla JS
- [Vue 客户端](/api/clients/vue) - Vue.js
- [API 文档](/api/overview) - 所有接口
