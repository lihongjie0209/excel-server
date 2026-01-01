# 样式定义

定义单元格的显示样式。

## 基本结构

```json
{
  "row": 0,
  "col": 0,
  "value": "标题",
  "value_type": "string",
  "style": {
    "bold": true,
    "italic": false,
    "underline": false,
    "font_size": 14,
    "font_color": "#FFFFFF",
    "bg_color": "#4472C4"
  }
}
```

## 字段说明

### bold

是否加粗。

- **类型**: Boolean
- **默认值**: `false`
- **示例**: `true`

### italic

是否斜体。

- **类型**: Boolean
- **默认值**: `false`
- **示例**: `true`

### underline

是否下划线。

- **类型**: Boolean
- **默认值**: `false`
- **示例**: `true`

### font_size

字体大小（磅）。

- **类型**: Integer
- **默认值**: `11`
- **范围**: `1-409`
- **示例**: `14`

### font_color

字体颜色（十六进制）。

- **类型**: String
- **格式**: `#RRGGBB`
- **默认值**: `#000000` (黑色)
- **示例**: `"#FF0000"` (红色)

### bg_color

背景颜色（十六进制）。

- **类型**: String
- **格式**: `#RRGGBB`
- **默认值**: 无背景色
- **示例**: `"#FFFF00"` (黄色)

## 常用样式

### 表头样式

```json
{
  "style": {
    "bold": true,
    "bg_color": "#4472C4",
    "font_color": "#FFFFFF",
    "font_size": 12
  }
}
```

### 标题样式

```json
{
  "style": {
    "bold": true,
    "font_size": 16,
    "font_color": "#2F75B5"
  }
}
```

### 警告样式

```json
{
  "style": {
    "bold": true,
    "font_color": "#FFFFFF",
    "bg_color": "#FF0000"
  }
}
```

### 成功样式

```json
{
  "style": {
    "font_color": "#00B050",
    "bg_color": "#E2EFDA"
  }
}
```

## 完整示例

```json
{
  "sheets": [{
    "name": "样式示例",
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
        "value": "状态",
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
        "value": "热销",
        "value_type": "string",
        "style": {
          "bold": true,
          "font_color": "#00B050"
        }
      }
    ]
  }]
}
```

## 颜色参考

### 常用颜色

| 颜色 | 十六进制 |
|------|---------|
| 黑色 | #000000 |
| 白色 | #FFFFFF |
| 红色 | #FF0000 |
| 绿色 | #00FF00 |
| 蓝色 | #0000FF |
| 黄色 | #FFFF00 |
| 橙色 | #FFA500 |
| 紫色 | #800080 |
| 灰色 | #808080 |

### Office 主题色

| 名称 | 十六进制 |
|------|---------|
| 深蓝 | #4472C4 |
| 浅蓝 | #5B9BD5 |
| 橙色 | #ED7D31 |
| 灰色 | #A5A5A5 |
| 黄色 | #FFC000 |
| 深绿 | #70AD47 |
| 浅绿 | #E2EFDA |

## 性能提示

```javascript
// ❌ 每个单元格不同样式（性能较差）
cells: data.map((item, index) => ({
  row: index,
  col: 0,
  value: item.name,
  value_type: "string",
  style: { bold: index === 0 }  // 动态样式
}))

// ✅ 复用相同样式（性能较好）
const headerStyle = { bold: true, bg_color: "#4472C4" };
cells: [
  { row: 0, col: 0, value: "Name", style: headerStyle },
  { row: 0, col: 1, value: "Age", style: headerStyle }
]
```

## 下一步

- [单元格](/dsl/cells) - 单元格定义
- [工作表](/dsl/worksheet) - 工作表配置
- [DSL 概述](/dsl/overview) - 返回概述
