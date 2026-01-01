# 测试脚本

## 1. 健康检查
curl http://localhost:3000/health

## 2. 直接生成 Excel（保存到文件）
curl -X POST http://localhost:3000/api/excel/generate \
  -H "Content-Type: application/json" \
  -d @examples/simple.json \
  --output simple_report.xlsx

## 3. 异步生成 Excel
$response = Invoke-RestMethod -Uri "http://localhost:3000/api/excel/async" `
  -Method POST `
  -ContentType "application/json" `
  -InFile "examples/simple.json"

$fileId = $response.data.file_id
Write-Host "文件 ID: $fileId"

## 4. 下载文件
Invoke-RestMethod -Uri "http://localhost:3000/api/excel/download" `
  -Method POST `
  -ContentType "application/json" `
  -Body (@{file_id=$fileId} | ConvertTo-Json) `
  -OutFile "downloaded.xlsx"

## 5. 查询存储状态
Invoke-RestMethod -Uri "http://localhost:3000/api/excel/status" `
  -Method POST `
  -ContentType "application/json"

## 6. 高级示例
curl -X POST http://localhost:3000/api/excel/generate \
  -H "Content-Type: application/json" \
  -d @examples/advanced.json \
  --output advanced_report.xlsx
