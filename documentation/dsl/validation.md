# 数据验证

当前版本暂不支持数据验证（Data Validation）功能。

## 当前状态

❌ 未实现

## 什么是数据验证

数据验证用于限制单元格可输入的值，例如：

- 下拉列表选择
- 数值范围限制
- 日期范围限制
- 自定义公式验证

## 计划支持

该功能计划在未来版本中支持。

示例（未来可能的 API）:

```json
{
  "sheets": [{
    "name": "Sheet1",
    "cells": [...],
    "validations": [
      {
        "range": "A1:A10",
        "type": "list",
        "options": ["选项1", "选项2", "选项3"]
      },
      {
        "range": "B1:B10",
        "type": "number",
        "min": 0,
        "max": 100
      }
    ]
  }]
}
```

## 下一步

- [单元格](/dsl/cells) - 单元格定义
- [工作表](/dsl/worksheet) - 工作表配置
- [DSL 概述](/dsl/overview) - 返回概述
