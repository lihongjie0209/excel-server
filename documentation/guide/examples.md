# 使用示例

## 基础示例

### 简单数据导出

```javascript
const dsl = {
  sheets: [{
    name: "用户列表",
    cells: [
      // 表头
      { row: 0, col: 0, value: "ID", value_type: "string" },
      { row: 0, col: 1, value: "姓名", value_type: "string" },
      { row: 0, col: 2, value: "年龄", value_type: "string" },
      // 数据行
      { row: 1, col: 0, value: 1, value_type: "number" },
      { row: 1, col: 1, value: "张三", value_type: "string" },
      { row: 1, col: 2, value: 25, value_type: "number" }
    ]
  }]
};

// 异步生成
const response = await fetch('/api/excel/async', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify(dsl)
});

const { data } = await response.json();
window.location.href = `/api/excel/download/${data.file_id}`;
```

### 带样式的表格

```javascript
const dsl = {
  sheets: [{
    name: "销售报表",
    cells: [
      // 标题行（加粗、背景色）
      {
        row: 0, col: 0, value: "产品", value_type: "string",
        style: { bold: true, bg_color: "#4472C4", font_color: "#FFFFFF" }
      },
      {
        row: 0, col: 1, value: "销量", value_type: "string",
        style: { bold: true, bg_color: "#4472C4", font_color: "#FFFFFF" }
      },
      {
        row: 0, col: 2, value: "金额", value_type: "string",
        style: { bold: true, bg_color: "#4472C4", font_color: "#FFFFFF" }
      },
      // 数据行
      { row: 1, col: 0, value: "iPhone 15", value_type: "string" },
      { row: 1, col: 1, value: 120, value_type: "number" },
      { 
        row: 1, col: 2, value: 96000, value_type: "number",
        style: { font_color: "#00B050" }  // 绿色表示正数
      }
    ],
    column_widths: {
      0: 20,
      1: 10,
      2: 15
    }
  }]
};
```

## 实际场景

### 1. 数据库导出

```javascript
async function exportUsers() {
  // 从数据库获取用户数据
  const users = await db.users.findAll();

  // 构建 DSL
  const dsl = {
    sheets: [{
      name: "用户列表",
      cells: [
        // 表头
        { row: 0, col: 0, value: "用户ID", value_type: "string", style: { bold: true } },
        { row: 0, col: 1, value: "用户名", value_type: "string", style: { bold: true } },
        { row: 0, col: 2, value: "邮箱", value_type: "string", style: { bold: true } },
        { row: 0, col: 3, value: "注册日期", value_type: "string", style: { bold: true } },
        // 数据行
        ...users.flatMap((user, index) => [
          { row: index + 1, col: 0, value: user.id, value_type: "number" },
          { row: index + 1, col: 1, value: user.username, value_type: "string" },
          { row: index + 1, col: 2, value: user.email, value_type: "string" },
          { row: index + 1, col: 3, value: user.created_at, value_type: "string" }
        ])
      ]
    }]
  };

  // 生成并下载
  const response = await fetch('/api/excel/async', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(dsl)
  });

  const result = await response.json();
  if (result.success) {
    window.location.href = `/api/excel/download/${result.data.file_id}`;
  }
}
```

### 2. 财务报表

```javascript
function generateFinancialReport(data) {
  return {
    sheets: [{
      name: "月度报表",
      cells: [
        // 标题
        { 
          row: 0, col: 0, value: "2026年1月财务报表", value_type: "string",
          style: { bold: true, font_size: 16 }
        },
        // 收入部分
        { row: 2, col: 0, value: "收入", value_type: "string", style: { bold: true } },
        { row: 3, col: 0, value: "产品销售", value_type: "string" },
        { row: 3, col: 1, value: data.productSales, value_type: "number" },
        { row: 4, col: 0, value: "服务收入", value_type: "string" },
        { row: 4, col: 1, value: data.serviceIncome, value_type: "number" },
        { row: 5, col: 0, value: "总收入", value_type: "string", style: { bold: true } },
        { 
          row: 5, col: 1, 
          value: data.productSales + data.serviceIncome, 
          value_type: "number",
          style: { bold: true, bg_color: "#E2EFDA" }
        },
        // 支出部分
        { row: 7, col: 0, value: "支出", value_type: "string", style: { bold: true } },
        { row: 8, col: 0, value: "运营成本", value_type: "string" },
        { row: 8, col: 1, value: data.operatingCost, value_type: "number" },
        { row: 9, col: 0, value: "人工成本", value_type: "string" },
        { row: 9, col: 1, value: data.laborCost, value_type: "number" },
        // 净利润
        { row: 11, col: 0, value: "净利润", value_type: "string", style: { bold: true } },
        {
          row: 11, col: 1,
          value: data.productSales + data.serviceIncome - data.operatingCost - data.laborCost,
          value_type: "number",
          style: { bold: true, bg_color: "#FFF2CC" }
        }
      ],
      column_widths: {
        0: 20,
        1: 15
      }
    }]
  };
}
```

### 3. 多工作表报告

```javascript
function generateMultiSheetReport(salesData, inventoryData) {
  return {
    sheets: [
      // 销售数据表
      {
        name: "销售数据",
        cells: salesData.flatMap((item, index) => [
          { row: index, col: 0, value: item.product, value_type: "string" },
          { row: index, col: 1, value: item.quantity, value_type: "number" },
          { row: index, col: 2, value: item.revenue, value_type: "number" }
        ])
      },
      // 库存数据表
      {
        name: "库存数据",
        cells: inventoryData.flatMap((item, index) => [
          { row: index, col: 0, value: item.product, value_type: "string" },
          { row: index, col: 1, value: item.stock, value_type: "number" },
          { row: index, col: 2, value: item.warehouse, value_type: "string" }
        ])
      }
    ],
    document_properties: {
      title: "月度报告",
      author: "财务部",
      subject: "销售与库存分析"
    }
  };
}
```

### 4. 动态列数表格

```javascript
function generateDynamicTable(headers, rows) {
  const cells = [];

  // 表头
  headers.forEach((header, colIndex) => {
    cells.push({
      row: 0,
      col: colIndex,
      value: header,
      value_type: "string",
      style: { bold: true, bg_color: "#4472C4", font_color: "#FFFFFF" }
    });
  });

  // 数据行
  rows.forEach((row, rowIndex) => {
    row.forEach((value, colIndex) => {
      cells.push({
        row: rowIndex + 1,
        col: colIndex,
        value: value,
        value_type: typeof value === 'number' ? 'number' : 'string'
      });
    });
  });

  return {
    sheets: [{
      name: "数据",
      cells: cells
    }]
  };
}

// 使用
const headers = ["姓名", "部门", "职位", "工资"];
const rows = [
  ["张三", "技术部", "工程师", 15000],
  ["李四", "市场部", "经理", 20000],
  ["王五", "人事部", "主管", 18000]
];

const dsl = generateDynamicTable(headers, rows);
```

### 5. 带公式的表格

```javascript
const dsl = {
  sheets: [{
    name: "销售统计",
    cells: [
      // 表头
      { row: 0, col: 0, value: "产品", value_type: "string", style: { bold: true } },
      { row: 0, col: 1, value: "单价", value_type: "string", style: { bold: true } },
      { row: 0, col: 2, value: "数量", value_type: "string", style: { bold: true } },
      { row: 0, col: 3, value: "小计", value_type: "string", style: { bold: true } },
      // 数据行
      { row: 1, col: 0, value: "产品A", value_type: "string" },
      { row: 1, col: 1, value: 100, value_type: "number" },
      { row: 1, col: 2, value: 10, value_type: "number" },
      { row: 1, col: 3, value: "=B2*C2", value_type: "formula" },  // 公式
      { row: 2, col: 0, value: "产品B", value_type: "string" },
      { row: 2, col: 1, value: 200, value_type: "number" },
      { row: 2, col: 2, value: 5, value_type: "number" },
      { row: 2, col: 3, value: "=B3*C3", value_type: "formula" },
      // 总计
      { row: 3, col: 0, value: "总计", value_type: "string", style: { bold: true } },
      { row: 3, col: 3, value: "=SUM(D2:D3)", value_type: "formula", style: { bold: true } }
    ]
  }]
};
```

## 错误处理

### 完整的错误处理流程

```javascript
async function safeGenerateExcel(dsl) {
  try {
    // 1. 生成文件
    const generateResp = await fetch('/api/excel/async', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(dsl)
    });

    const result = await generateResp.json();

    // 2. 检查业务状态
    if (!result.success) {
      switch (result.code) {
        case 1001:
          throw new Error('参数错误: ' + result.message);
        case 2001:
          throw new Error('Excel 生成失败: ' + result.message);
        default:
          throw new Error('未知错误: ' + result.message);
      }
    }

    // 3. 下载文件
    const fileId = result.data.file_id;
    const downloadResp = await fetch(`/api/excel/download/${fileId}`);

    if (!downloadResp.ok) {
      const error = await downloadResp.json();
      throw new Error('下载失败: ' + error.message);
    }

    // 4. 保存文件
    const blob = await downloadResp.blob();
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'report.xlsx';
    a.click();
    window.URL.revokeObjectURL(url);

    return { success: true };
  } catch (error) {
    console.error('Excel 生成失败:', error);
    return { success: false, error: error.message };
  }
}
```

## React 完整示例

```jsx
import React, { useState } from 'react';

function ExcelExporter({ data }) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const exportData = async () => {
    setLoading(true);
    setError(null);

    // 构建 DSL
    const dsl = {
      sheets: [{
        name: "数据导出",
        cells: [
          // 表头
          { row: 0, col: 0, value: "ID", value_type: "string", style: { bold: true } },
          { row: 0, col: 1, value: "名称", value_type: "string", style: { bold: true } },
          { row: 0, col: 2, value: "数值", value_type: "string", style: { bold: true } },
          // 数据
          ...data.flatMap((item, index) => [
            { row: index + 1, col: 0, value: item.id, value_type: "number" },
            { row: index + 1, col: 1, value: item.name, value_type: "string" },
            { row: index + 1, col: 2, value: item.value, value_type: "number" }
          ])
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

      if (!result.success) {
        throw new Error(result.message);
      }

      window.location.href = `/api/excel/download/${result.data.file_id}`;
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <button onClick={exportData} disabled={loading}>
        {loading ? '导出中...' : '导出 Excel'}
      </button>
      {error && <div style={{ color: 'red' }}>{error}</div>}
    </div>
  );
}

export default ExcelExporter;
```

## 下一步

- [API 文档](/api/overview) - 了解所有接口
- [DSL 规范](/dsl/overview) - 深入学习 DSL
- [配置说明](/guide/configuration) - 自定义配置
