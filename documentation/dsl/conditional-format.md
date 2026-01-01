# 条件格式

当前版本暂不支持条件格式（Conditional Formatting）功能。

## 当前状态

❌ 未实现

## 什么是条件格式

条件格式根据单元格的值自动应用不同的格式，例如：

- 数据条（Data Bars）
- 色阶（Color Scales）
- 图标集（Icon Sets）
- 条件规则（Rules）

## 替代方案

在生成数据时根据条件预先设置样式：

```javascript
const cells = data.map((item, index) => ({
  row: index + 1,
  col: 0,
  value: item.value,
  value_type: "number",
  style: {
    // 根据值设置颜色
    font_color: item.value > 100 ? "#00B050" : "#FF0000",
    bg_color: item.value > 100 ? "#E2EFDA" : "#FFC7CE"
  }
}));
```

## 计划支持

该功能计划在未来版本中支持。

示例（未来可能的 API）:

```json
{
  "sheets": [{
    "name": "Sheet1",
    "cells": [...],
    "conditional_formats": [
      {
        "range": "A1:A10",
        "type": "data_bar",
        "color": "#4472C4"
      },
      {
        "range": "B1:B10",
        "type": "color_scale",
        "min_color": "#FF0000",
        "max_color": "#00FF00"
      },
      {
        "range": "C1:C10",
        "type": "rule",
        "condition": "greater_than",
        "value": 100,
        "format": {
          "bg_color": "#00B050",
          "font_color": "#FFFFFF"
        }
      }
    ]
  }]
}
```

## 下一步

- [单元格](/dsl/cells) - 单元格定义
- [样式定义](/dsl/styles) - 样式配置
- [DSL 概述](/dsl/overview) - 返回概述
