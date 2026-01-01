# 数据表格

当前版本暂不支持表格（Table）功能。

## 当前状态

❌ 未实现

## 替代方案

使用单元格配合样式实现类似表格的效果：

```json
{
  "sheets": [{
    "name": "数据表",
    "cells": [
      // 表头（加样式）
      {
        "row": 0, "col": 0,
        "value": "ID",
        "value_type": "string",
        "style": {
          "bold": true,
          "bg_color": "#4472C4",
          "font_color": "#FFFFFF"
        }
      },
      {
        "row": 0, "col": 1,
        "value": "姓名",
        "value_type": "string",
        "style": {
          "bold": true,
          "bg_color": "#4472C4",
          "font_color": "#FFFFFF"
        }
      },
      // 数据行
      { "row": 1, "col": 0, "value": 1, "value_type": "number" },
      { "row": 1, "col": 1, "value": "张三", "value_type": "string" }
    ],
    "column_widths": {
      "0": 10,
      "1": 20
    }
  }]
}
```

## 计划支持

该功能计划在未来版本中支持。

## 下一步

- [单元格](/dsl/cells) - 单元格定义
- [样式定义](/dsl/styles) - 样式配置
- [DSL 概述](/dsl/overview) - 返回概述
