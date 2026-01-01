# DSL 规范概述

Excel Server 使用自定义的 DSL (Domain Specific Language) 来描述 Excel 文档结构。

## DSL 版本

当前支持版本: **v1.3**

## 基本结构

一个完整的 Excel DSL 文档包含以下部分：

```json
{
  "properties": {
    // 文档属性
  },
  "styles": {
    // 样式定义
  },
  "sheets": [
    // 工作表数组
  ]
}
```

## 最小示例

最简单的 Excel 文档：

```json
{
  "sheets": [
    {
      "name": "Sheet1",
      "cells": [
        {
          "r": 0,
          "c": 0,
          "type": "string",
          "value": "Hello World"
        }
      ]
    }
  ]
}
```

## 完整示例

包含样式、公式、合并单元格的完整示例：

```json
{
  "properties": {
    "title": "销售报表",
    "author": "张三",
    "company": "示例公司"
  },
  "styles": {
    "header": {
      "font": {
        "bold": true,
        "size": 14,
        "color": "#FFFFFF"
      },
      "fill": {
        "color": "#4472C4"
      },
      "align": {
        "h": "center",
        "v": "vcenter"
      }
    }
  },
  "sheets": [
    {
      "name": "销售数据",
      "cells": [
        {
          "r": 0,
          "c": 0,
          "type": "string",
          "value": "产品",
          "style": "header"
        },
        {
          "r": 0,
          "c": 1,
          "type": "string",
          "value": "销售额",
          "style": "header"
        },
        {
          "r": 1,
          "c": 0,
          "type": "string",
          "value": "产品A"
        },
        {
          "r": 1,
          "c": 1,
          "type": "number",
          "value": 12500.50
        }
      ],
      "merges": [
        { "range": "A1:B1" }
      ]
    }
  ]
}
```

## DSL 主要组成部分

### 1. 文档属性 (properties)

定义 Excel 文档的元数据：

- `title` - 标题
- `author` - 作者
- `company` - 公司
- `subject` - 主题
- `category` - 类别
- `keywords` - 关键词
- `comments` - 备注

[详细说明 →](/dsl/document-properties)

### 2. 样式定义 (styles)

定义可复用的样式：

- `font` - 字体样式
- `fill` - 填充样式
- `align` - 对齐样式
- `border` - 边框样式
- `protect` - 保护样式

[详细说明 →](/dsl/styles)

### 3. 工作表 (sheets)

包含工作表的所有内容：

- `name` - 工作表名称
- `cells` - 单元格数组
- `merges` - 合并单元格
- `tables` - 数据表格
- `validations` - 数据验证
- `conditional_formats` - 条件格式

[详细说明 →](/dsl/worksheet)

## 坐标系统

Excel Server 支持两种坐标表示方法：

### 1. 数字坐标（推荐）

```json
{
  "r": 0,  // 行号（从 0 开始）
  "c": 0   // 列号（从 0 开始）
}
```

### 2. A1 引用

```json
{
  "range": "A1:B10"  // Excel A1 格式
}
```

也可以在 `RangeCoords` 中使用：

```json
{
  "start_r": 0,
  "start_c": 0,
  "end_r": 9,
  "end_c": 1
}
```

## 数据类型

### 单元格类型

- `string` - 字符串
- `number` - 数字
- `formula` - 公式
- `datetime` - 日期时间
- `blank` - 空白

[详细说明 →](/dsl/cells)

### 公式示例

```json
{
  "r": 2,
  "c": 2,
  "type": "formula",
  "value": "=SUM(A1:A10)"
}
```

### 日期时间示例

```json
{
  "r": 0,
  "c": 0,
  "type": "datetime",
  "value": "2026-01-01T10:30:00"
}
```

## 高级功能

### 数据表格

创建带有格式化的表格：

```json
{
  "table": {
    "name": "SalesTable",
    "range": "A1:C10",
    "columns": [
      { "name": "Product" },
      { "name": "Quantity" },
      { "name": "Price" }
    ]
  }
}
```

[详细说明 →](/dsl/tables)

### 数据验证

限制单元格输入：

```json
{
  "validation": {
    "range": "A2:A100",
    "type": "list",
    "value": ["选项1", "选项2", "选项3"]
  }
}
```

[详细说明 →](/dsl/validation)

### 条件格式

根据条件应用格式：

```json
{
  "conditional_format": {
    "range": "B2:B100",
    "type": "cell",
    "condition": ">100",
    "style": "highlight"
  }
}
```

[详细说明 →](/dsl/conditional-format)

## 最佳实践

### 1. 使用样式复用

❌ 不推荐 - 重复定义样式：

```json
{
  "cells": [
    {
      "r": 0,
      "c": 0,
      "value": "标题1",
      "style": {
        "font": { "bold": true, "size": 14 }
      }
    },
    {
      "r": 0,
      "c": 1,
      "value": "标题2",
      "style": {
        "font": { "bold": true, "size": 14 }
      }
    }
  ]
}
```

✅ 推荐 - 定义并复用样式：

```json
{
  "styles": {
    "header": {
      "font": { "bold": true, "size": 14 }
    }
  },
  "sheets": [{
    "cells": [
      { "r": 0, "c": 0, "value": "标题1", "style": "header" },
      { "r": 0, "c": 1, "value": "标题2", "style": "header" }
    ]
  }]
}
```

### 2. 使用数字坐标

数字坐标更容易程序化生成：

```javascript
// 生成 10x10 的表格
const cells = [];
for (let r = 0; r < 10; r++) {
  for (let c = 0; c < 10; c++) {
    cells.push({
      r, c,
      type: 'number',
      value: r * 10 + c
    });
  }
}
```

### 3. 合理使用公式

- 使用相对引用而非绝对引用
- 避免循环引用
- 复杂计算在服务端完成

## 验证工具

使用 Swagger UI 验证 DSL 格式：

```bash
# 访问 Swagger UI
open http://localhost:3000/swagger-ui/

# 使用 "Try it out" 功能测试 DSL
```

## 下一步

- [文档属性](/dsl/document-properties) - 设置文档元数据
- [样式定义](/dsl/styles) - 创建美观的样式
- [单元格](/dsl/cells) - 填充数据
- [工作表](/dsl/worksheet) - 组织工作表结构

## 示例文件

查看完整示例：

- [简单示例](https://github.com/lihongjie0209/excel-server/blob/main/examples/simple.json)
- [高级示例](https://github.com/lihongjie0209/excel-server/blob/main/examples/advanced.json)

