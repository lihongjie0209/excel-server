# 单元格

定义 Excel 单元格的内容和格式。

## 基本结构

```json
{
  "row": 0,
  "col": 0,
  "value": "Hello",
  "value_type": "string",
  "style": {...}
}
```

## 字段说明

### row

行索引（从 0 开始）。

- **类型**: Integer
- **必填**: ✅
- **范围**: `0-1048575`
- **示例**: `0` (第 1 行), `99` (第 100 行)

### col

列索引（从 0 开始）。

- **类型**: Integer
- **必填**: ✅
- **范围**: `0-16383`
- **示例**: `0` (A 列), `1` (B 列), `25` (Z 列)

### value

单元格的值。

- **类型**: String | Number | Boolean
- **必填**: ✅
- **说明**: 根据 `value_type` 确定类型

### value_type

值的类型。

- **类型**: Enum
- **必填**: ✅
- **可选值**:
  - `"string"` - 文本
  - `"number"` - 数字
  - `"boolean"` - 布尔值
  - `"formula"` - 公式

### style

单元格样式（可选）。

- **类型**: Object
- **必填**: ❌
- **说明**: 详见 [样式定义](/dsl/styles)

## 数据类型示例

### 字符串 (string)

```json
{
  "row": 0,
  "col": 0,
  "value": "Hello World",
  "value_type": "string"
}
```

### 数字 (number)

```json
{
  "row": 0,
  "col": 0,
  "value": 12345.67,
  "value_type": "number"
}
```

### 布尔值 (boolean)

```json
{
  "row": 0,
  "col": 0,
  "value": true,
  "value_type": "boolean"
}
```

### 公式 (formula)

```json
{
  "row": 0,
  "col": 0,
  "value": "=SUM(A1:A10)",
  "value_type": "formula"
}
```

## 完整示例

### 基础数据表

```json
{
  "sheets": [{
    "name": "数据",
    "cells": [
      // 表头
      { "row": 0, "col": 0, "value": "姓名", "value_type": "string" },
      { "row": 0, "col": 1, "value": "年龄", "value_type": "string" },
      { "row": 0, "col": 2, "value": "是否会员", "value_type": "string" },
      
      // 数据行
      { "row": 1, "col": 0, "value": "张三", "value_type": "string" },
      { "row": 1, "col": 1, "value": 25, "value_type": "number" },
      { "row": 1, "col": 2, "value": true, "value_type": "boolean" },
      
      { "row": 2, "col": 0, "value": "李四", "value_type": "string" },
      { "row": 2, "col": 1, "value": 30, "value_type": "number" },
      { "row": 2, "col": 2, "value": false, "value_type": "boolean" }
    ]
  }]
}
```

### 带样式的单元格

```json
{
  "sheets": [{
    "name": "报表",
    "cells": [
      {
        "row": 0,
        "col": 0,
        "value": "产品",
        "value_type": "string",
        "style": {
          "bold": true,
          "bg_color": "#4472C4",
          "font_color": "#FFFFFF"
        }
      },
      {
        "row": 0,
        "col": 1,
        "value": "销量",
        "value_type": "string",
        "style": {
          "bold": true,
          "bg_color": "#4472C4",
          "font_color": "#FFFFFF"
        }
      },
      {
        "row": 1,
        "col": 0,
        "value": "iPhone 15",
        "value_type": "string"
      },
      {
        "row": 1,
        "col": 1,
        "value": 120,
        "value_type": "number",
        "style": {
          "font_color": "#00B050"
        }
      }
    ]
  }]
}
```

### 使用公式

```json
{
  "sheets": [{
    "name": "计算",
    "cells": [
      // 数据
      { "row": 0, "col": 0, "value": "数量", "value_type": "string" },
      { "row": 0, "col": 1, "value": "单价", "value_type": "string" },
      { "row": 0, "col": 2, "value": "小计", "value_type": "string" },
      
      { "row": 1, "col": 0, "value": 10, "value_type": "number" },
      { "row": 1, "col": 1, "value": 99.9, "value_type": "number" },
      { "row": 1, "col": 2, "value": "=A2*B2", "value_type": "formula" },
      
      { "row": 2, "col": 0, "value": 5, "value_type": "number" },
      { "row": 2, "col": 1, "value": 149.9, "value_type": "number" },
      { "row": 2, "col": 2, "value": "=A3*B3", "value_type": "formula" },
      
      // 总计
      { "row": 3, "col": 0, "value": "总计", "value_type": "string" },
      { "row": 3, "col": 2, "value": "=SUM(C2:C3)", "value_type": "formula" }
    ]
  }]
}
```

## 常用公式

### 求和

```json
{ "value": "=SUM(A1:A10)", "value_type": "formula" }
```

### 平均值

```json
{ "value": "=AVERAGE(B1:B10)", "value_type": "formula" }
```

### 计数

```json
{ "value": "=COUNT(C1:C10)", "value_type": "formula" }
```

### 条件求和

```json
{ "value": "=SUMIF(A:A,\">100\",B:B)", "value_type": "formula" }
```

### IF 函数

```json
{ "value": "=IF(A1>100,\"合格\",\"不合格\")", "value_type": "formula" }
```

## 坐标转换

### 索引 → Excel 坐标

| row | col | Excel 坐标 |
|-----|-----|-----------|
| 0 | 0 | A1 |
| 0 | 1 | B1 |
| 1 | 0 | A2 |
| 9 | 25 | Z10 |

### JavaScript 转换函数

```javascript
function toExcelCoord(row, col) {
  const colName = String.fromCharCode(65 + col);
  return `${colName}${row + 1}`;
}

console.log(toExcelCoord(0, 0));  // "A1"
console.log(toExcelCoord(9, 2));  // "C10"
```

## 动态生成

### JavaScript

```javascript
function createCells(data) {
  return data.flatMap((row, rowIndex) =>
    row.map((value, colIndex) => ({
      row: rowIndex,
      col: colIndex,
      value: value,
      value_type: typeof value === 'number' ? 'number' : 
                  typeof value === 'boolean' ? 'boolean' : 'string'
    }))
  );
}

// 使用
const data = [
  ["姓名", "年龄", "会员"],
  ["张三", 25, true],
  ["李四", 30, false]
];

const cells = createCells(data);
```

### Python

```python
def create_cells(data: list) -> list:
    cells = []
    for row_idx, row in enumerate(data):
        for col_idx, value in enumerate(row):
            value_type = (
                "number" if isinstance(value, (int, float)) else
                "boolean" if isinstance(value, bool) else
                "string"
            )
            cells.append({
                "row": row_idx,
                "col": col_idx,
                "value": value,
                "value_type": value_type
            })
    return cells

# 使用
data = [
    ["姓名", "年龄", "会员"],
    ["张三", 25, True],
    ["李四", 30, False]
]

cells = create_cells(data)
```

## 性能优化

### 批量生成

```javascript
// ✅ 推荐：一次性生成所有单元格
const cells = [];
for (let row = 0; row < 100; row++) {
  for (let col = 0; col < 10; col++) {
    cells.push({
      row, col,
      value: `R${row}C${col}`,
      value_type: "string"
    });
  }
}

// ❌ 避免：逐个发送请求
for (let row = 0; row < 100; row++) {
  await generateExcel({ cells: [...] });  // 低效
}
```

## 下一步

- [样式定义](/dsl/styles) - 单元格样式
- [工作表](/dsl/worksheet) - 工作表配置
- [DSL 概述](/dsl/overview) - 返回概述
