# cURL 示例

## 基础用法

### 同步生成

```bash
curl -X POST http://localhost:3000/api/excel/generate \
  -H "Content-Type: application/json" \
  -d '{
    "sheets": [{
      "name": "Sheet1",
      "cells": [
        {"row": 0, "col": 0, "value": "Hello", "value_type": "string"}
      ]
    }]
  }' \
  --output output.xlsx
```

### 异步生成 + 下载

```bash
# 1. 生成文件，提取 file_id
FILE_ID=$(curl -X POST http://localhost:3000/api/excel/async \
  -H "Content-Type: application/json" \
  -d '{
    "sheets": [{
      "name": "Sheet1",
      "cells": [
        {"row": 0, "col": 0, "value": "Hello", "value_type": "string"}
      ]
    }]
  }' | jq -r '.data.file_id')

# 2. 下载文件
curl http://localhost:3000/api/excel/download/${FILE_ID} --output report.xlsx
```

## 从文件读取 DSL

### 准备 JSON 文件

```bash
# dsl.json
cat > dsl.json << 'EOF'
{
  "sheets": [{
    "name": "用户列表",
    "cells": [
      {"row": 0, "col": 0, "value": "ID", "value_type": "string"},
      {"row": 0, "col": 1, "value": "姓名", "value_type": "string"},
      {"row": 1, "col": 0, "value": 1, "value_type": "number"},
      {"row": 1, "col": 1, "value": "张三", "value_type": "string"}
    ]
  }]
}
EOF
```

### 使用文件

```bash
curl -X POST http://localhost:3000/api/excel/generate \
  -H "Content-Type: application/json" \
  -d @dsl.json \
  --output users.xlsx
```

## 复杂示例

### 带样式的表格

```bash
curl -X POST http://localhost:3000/api/excel/async \
  -H "Content-Type: application/json" \
  -d '{
    "sheets": [{
      "name": "销售报表",
      "cells": [
        {
          "row": 0, "col": 0, 
          "value": "产品", 
          "value_type": "string",
          "style": {
            "bold": true,
            "bg_color": "#4472C4",
            "font_color": "#FFFFFF"
          }
        },
        {
          "row": 0, "col": 1,
          "value": "销量",
          "value_type": "string",
          "style": {
            "bold": true,
            "bg_color": "#4472C4",
            "font_color": "#FFFFFF"
          }
        },
        {
          "row": 1, "col": 0,
          "value": "iPhone 15",
          "value_type": "string"
        },
        {
          "row": 1, "col": 1,
          "value": 120,
          "value_type": "number"
        }
      ],
      "column_widths": {
        "0": 20,
        "1": 10
      }
    }]
  }'
```

### 多工作表

```bash
curl -X POST http://localhost:3000/api/excel/generate \
  -H "Content-Type: application/json" \
  -d '{
    "sheets": [
      {
        "name": "Sheet1",
        "cells": [
          {"row": 0, "col": 0, "value": "数据1", "value_type": "string"}
        ]
      },
      {
        "name": "Sheet2",
        "cells": [
          {"row": 0, "col": 0, "value": "数据2", "value_type": "string"}
        ]
      }
    ]
  }' \
  --output multi-sheet.xlsx
```

## 完整脚本

### Bash 脚本

```bash
#!/bin/bash

# generate-excel.sh
set -e

API_BASE="http://localhost:3000"
DSL_FILE="$1"
OUTPUT_FILE="$2"

if [ -z "$DSL_FILE" ] || [ -z "$OUTPUT_FILE" ]; then
  echo "Usage: $0 <dsl.json> <output.xlsx>"
  exit 1
fi

echo "Generating Excel..."
RESPONSE=$(curl -s -X POST "${API_BASE}/api/excel/async" \
  -H "Content-Type: application/json" \
  -d @"${DSL_FILE}")

SUCCESS=$(echo "$RESPONSE" | jq -r '.success')

if [ "$SUCCESS" != "true" ]; then
  echo "Error: $(echo "$RESPONSE" | jq -r '.message')"
  exit 1
fi

FILE_ID=$(echo "$RESPONSE" | jq -r '.data.file_id')
echo "File ID: $FILE_ID"

echo "Downloading..."
curl -s "${API_BASE}/api/excel/download/${FILE_ID}" --output "${OUTPUT_FILE}"

echo "Done: ${OUTPUT_FILE}"
```

**使用**:
```bash
chmod +x generate-excel.sh
./generate-excel.sh dsl.json report.xlsx
```

### PowerShell 脚本

```powershell
# generate-excel.ps1
param(
    [Parameter(Mandatory=$true)]
    [string]$DslFile,
    
    [Parameter(Mandatory=$true)]
    [string]$OutputFile
)

$ApiBase = "http://localhost:3000"
$Dsl = Get-Content $DslFile -Raw

Write-Host "Generating Excel..."
$Response = Invoke-RestMethod -Uri "$ApiBase/api/excel/async" `
    -Method Post `
    -ContentType "application/json" `
    -Body $Dsl

if (-not $Response.success) {
    Write-Error "Error: $($Response.message)"
    exit 1
}

$FileId = $Response.data.file_id
Write-Host "File ID: $FileId"

Write-Host "Downloading..."
Invoke-WebRequest -Uri "$ApiBase/api/excel/download/$FileId" `
    -OutFile $OutputFile

Write-Host "Done: $OutputFile"
```

**使用**:
```powershell
.\generate-excel.ps1 -DslFile dsl.json -OutputFile report.xlsx
```

## 批量生成

```bash
#!/bin/bash

# 批量生成多个报表
for i in {1..10}; do
  FILE_ID=$(curl -s -X POST http://localhost:3000/api/excel/async \
    -H "Content-Type: application/json" \
    -d "{
      \"sheets\": [{
        \"name\": \"Report$i\",
        \"cells\": [
          {\"row\": 0, \"col\": 0, \"value\": \"Report $i\", \"value_type\": \"string\"}
        ]
      }]
    }" | jq -r '.data.file_id')
  
  curl -s "http://localhost:3000/api/excel/download/${FILE_ID}" \
    --output "report_${i}.xlsx"
  
  echo "Generated report_${i}.xlsx"
done
```

## 错误处理

```bash
#!/bin/bash

RESPONSE=$(curl -s -X POST http://localhost:3000/api/excel/async \
  -H "Content-Type: application/json" \
  -d @dsl.json)

# 检查成功标志
SUCCESS=$(echo "$RESPONSE" | jq -r '.success')

if [ "$SUCCESS" = "true" ]; then
  FILE_ID=$(echo "$RESPONSE" | jq -r '.data.file_id')
  echo "Success: $FILE_ID"
else
  CODE=$(echo "$RESPONSE" | jq -r '.code')
  MESSAGE=$(echo "$RESPONSE" | jq -r '.message')
  echo "Error [$CODE]: $MESSAGE"
  exit 1
fi
```

## 测试接口

### 健康检查

```bash
curl http://localhost:3000/health
# 输出: OK
```

### 存储状态

```bash
curl -X POST http://localhost:3000/api/excel/status \
  -H "Content-Type: application/json" | jq
```

### 性能测试

```bash
# Apache Bench
ab -n 1000 -c 50 -p dsl.json -T application/json \
  http://localhost:3000/api/excel/async

# 结果
# Requests per second:    856.47 [#/sec]
# Time per request:       58.391 [ms] (mean)
```

## 下一步

- [JavaScript 客户端](/api/clients/javascript) - 浏览器使用
- [API 概览](/api/overview) - 所有接口
- [快速开始](/guide/getting-started) - 开始使用
