# 文档属性

设置 Excel 文档的元数据信息。

## 基本结构

```json
{
  "sheets": [...],
  "document_properties": {
    "title": "月度报表",
    "author": "张三",
    "subject": "销售数据分析",
    "company": "示例公司",
    "category": "报表",
    "keywords": "销售,统计,月度",
    "comments": "这是2026年1月的销售报表"
  }
}
```

## 字段说明

### title

文档标题。

- **类型**: String
- **必填**: ❌
- **示例**: `"2026年1月销售报表"`

### author

作者姓名。

- **类型**: String
- **必填**: ❌
- **示例**: `"张三"`

### subject

主题或副标题。

- **类型**: String
- **必填**: ❌
- **示例**: `"销售数据统计与分析"`

### company

公司名称。

- **类型**: String
- **必填**: ❌
- **示例**: `"示例科技有限公司"`

### category

文档分类。

- **类型**: String
- **必填**: ❌
- **示例**: `"财务报表"`

### keywords

关键词（用逗号分隔）。

- **类型**: String
- **必填**: ❌
- **示例**: `"销售,统计,月度,2026"`

### comments

备注或描述。

- **类型**: String
- **必填**: ❌
- **示例**: `"本报表包含所有销售渠道的数据"`

## 完整示例

```json
{
  "sheets": [
    {
      "name": "销售数据",
      "cells": [
        { "row": 0, "col": 0, "value": "产品", "value_type": "string" },
        { "row": 0, "col": 1, "value": "销量", "value_type": "string" }
      ]
    }
  ],
  "document_properties": {
    "title": "2026年1月销售报表",
    "author": "财务部",
    "subject": "月度销售数据统计",
    "company": "示例科技有限公司",
    "category": "销售报表",
    "keywords": "销售,统计,月报,2026年1月",
    "comments": "包含所有产品线的销售数据"
  }
}
```

## 查看文档属性

在 Excel 中查看：

1. **文件** → **信息** → **属性**
2. 查看"标题"、"作者"等字段

## 下一步

- [样式定义](/dsl/styles) - 单元格样式
- [工作表](/dsl/worksheet) - 工作表配置
- [DSL 概述](/dsl/overview) - 返回概述
