# 工作表

定义 Excel 工作表（Sheet）的结构和属性。

## 基本结构

```json
{
  "name": "Sheet1",
  "cells": [...],
  "column_widths": {
    "0": 20,
    "1": 15
  },
  "row_heights": {
    "0": 25
  }
}
```

## 字段说明

### name

工作表名称。

- **类型**: String
- **必填**: ✅
- **限制**: 
  - 最长 31 字符
  - 不能包含: `\ / ? * [ ]`
  - 不能为空
- **示例**: `"销售数据"`, `"Sheet1"`

### cells

单元格数组。

- **类型**: Array[Cell]
- **必填**: ✅
- **说明**: 详见 [单元格](/dsl/cells) 文档

### column_widths

列宽设置（字符宽度）。

- **类型**: Object<Integer, Number>
- **必填**: ❌
- **格式**: `{ "列索引": 宽度 }`
- **默认值**: 8.43 字符
- **示例**: 
```json
{
  "0": 20,  // A 列宽度 20
  "1": 15   // B 列宽度 15
}
```

### row_heights

行高设置（磅）。

- **类型**: Object<Integer, Number>
- **必填**: ❌
- **格式**: `{ "行索引": 高度 }`
- **默认值**: 15 磅
- **示例**:
```json
{
  "0": 25,  // 第 1 行高度 25
  "1": 20   // 第 2 行高度 20
}
```

## 完整示例

### 基础工作表

```json
{
  "sheets": [{
    "name": "数据",
    "cells": [
      { "row": 0, "col": 0, "value": "姓名", "value_type": "string" },
      { "row": 0, "col": 1, "value": "年龄", "value_type": "string" },
      { "row": 1, "col": 0, "value": "张三", "value_type": "string" },
      { "row": 1, "col": 1, "value": 25, "value_type": "number" }
    ]
  }]
}
```

### 带列宽和行高

```json
{
  "sheets": [{
    "name": "格式化数据",
    "cells": [
      { "row": 0, "col": 0, "value": "ID", "value_type": "string" },
      { "row": 0, "col": 1, "value": "姓名", "value_type": "string" },
      { "row": 0, "col": 2, "value": "邮箱", "value_type": "string" }
    ],
    "column_widths": {
      "0": 10,   // ID 列
      "1": 20,   // 姓名列
      "2": 35    // 邮箱列
    },
    "row_heights": {
      "0": 25    // 表头行高
    }
  }]
}
```

### 多工作表

```json
{
  "sheets": [
    {
      "name": "销售数据",
      "cells": [
        { "row": 0, "col": 0, "value": "产品", "value_type": "string" },
        { "row": 0, "col": 1, "value": "销量", "value_type": "string" }
      ]
    },
    {
      "name": "库存数据",
      "cells": [
        { "row": 0, "col": 0, "value": "产品", "value_type": "string" },
        { "row": 0, "col": 1, "value": "库存", "value_type": "string" }
      ]
    }
  ]
}
```

## 列宽参考

| 内容类型 | 建议宽度 |
|---------|---------|
| ID (数字) | 8-10 |
| 姓名 | 15-20 |
| 邮箱 | 30-40 |
| 手机号 | 15 |
| 日期 | 12-15 |
| 金额 | 12-15 |
| 描述文本 | 30-50 |

## 行高参考

| 内容类型 | 建议高度 |
|---------|---------|
| 普通文本 | 15 (默认) |
| 表头 | 20-25 |
| 标题 | 25-30 |
| 多行文本 | 30+ |

## 动态生成示例

### JavaScript

```javascript
function createSheet(name, data) {
  return {
    name: name,
    cells: data.flatMap((row, rowIndex) =>
      row.map((value, colIndex) => ({
        row: rowIndex,
        col: colIndex,
        value: value,
        value_type: typeof value === 'number' ? 'number' : 'string'
      }))
    ),
    column_widths: {
      0: 20,
      1: 15,
      2: 25
    }
  };
}

// 使用
const sheet = createSheet("用户列表", [
  ["姓名", "年龄", "邮箱"],
  ["张三", 25, "zhangsan@example.com"],
  ["李四", 30, "lisi@example.com"]
]);
```

### Python

```python
def create_sheet(name: str, data: list) -> dict:
    cells = []
    for row_idx, row in enumerate(data):
        for col_idx, value in enumerate(row):
            cells.append({
                "row": row_idx,
                "col": col_idx,
                "value": value,
                "value_type": "number" if isinstance(value, (int, float)) else "string"
            })
    
    return {
        "name": name,
        "cells": cells,
        "column_widths": {
            "0": 20,
            "1": 15,
            "2": 25
        }
    }

# 使用
sheet = create_sheet("用户列表", [
    ["姓名", "年龄", "邮箱"],
    ["张三", 25, "zhangsan@example.com"],
    ["李四", 30, "lisi@example.com"]
])
```

## 注意事项

### 工作表名称限制

```javascript
// ✅ 有效名称
"Sheet1"
"销售数据"
"2026-01-report"

// ❌ 无效名称
"Sales/Report"      // 包含 /
"Data[2026]"        // 包含 []
""                  // 空名称
"Very Long Sheet Name Over 31 Chars"  // 超过 31 字符
```

### 性能优化

```javascript
// ✅ 推荐：仅设置需要的列宽
column_widths: {
  0: 20,
  2: 30
}

// ❌ 避免：设置所有列（即使是默认值）
column_widths: {
  0: 20,
  1: 8.43,  // 默认值，无需设置
  2: 30,
  3: 8.43   // 默认值，无需设置
}
```

## 下一步

- [单元格](/dsl/cells) - 单元格定义
- [样式定义](/dsl/styles) - 样式配置
- [DSL 概述](/dsl/overview) - 返回概述
