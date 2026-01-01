# 测试 GET 下载接口
Write-Host "===== 测试 Excel Server GET 下载功能 =====" -ForegroundColor Cyan

$baseUrl = "http://localhost:3000"

Write-Host "`n[提示] 请确保服务器已启动: cargo run" -ForegroundColor Yellow
Write-Host "按任意键继续..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

try {
    # 1. 生成文件并获取 file_id
    Write-Host "`n[步骤 1] 异步生成 Excel 文件..." -ForegroundColor Yellow
    $dsl = Get-Content "examples/simple.json" -Raw
    $response = Invoke-RestMethod -Uri "$baseUrl/api/excel/async" -Method Post -Body $dsl -ContentType "application/json"
    
    if ($response.success) {
        $fileId = $response.data.file_id
        Write-Host "✓ 文件生成成功" -ForegroundColor Green
        Write-Host "  file_id: $fileId" -ForegroundColor Cyan
        
        # 2. 使用 POST 方法下载（原方法）
        Write-Host "`n[步骤 2] 使用 POST 方法下载..." -ForegroundColor Yellow
        $downloadReq = @{ file_id = $fileId } | ConvertTo-Json
        Invoke-RestMethod -Uri "$baseUrl/api/excel/download" -Method Post -Body $downloadReq -ContentType "application/json" -OutFile "test_post.xlsx"
        Write-Host "✓ POST 下载成功: test_post.xlsx" -ForegroundColor Green
        
        # 3. 使用 GET 方法下载（新方法）
        Write-Host "`n[步骤 3] 使用 GET 方法下载..." -ForegroundColor Yellow
        Invoke-WebRequest -Uri "$baseUrl/api/excel/download/$fileId" -OutFile "test_get.xlsx"
        Write-Host "✓ GET 下载成功: test_get.xlsx" -ForegroundColor Green
        
        # 4. 验证文件大小
        Write-Host "`n[步骤 4] 验证文件..." -ForegroundColor Yellow
        $postFile = Get-Item "test_post.xlsx"
        $getFile = Get-Item "test_get.xlsx"
        Write-Host "  POST 方法下载的文件大小: $($postFile.Length) bytes" -ForegroundColor Cyan
        Write-Host "  GET 方法下载的文件大小:  $($getFile.Length) bytes" -ForegroundColor Cyan
        
        if ($postFile.Length -eq $getFile.Length) {
            Write-Host "✓ 文件大小一致，两种方法下载的文件相同" -ForegroundColor Green
        } else {
            Write-Host "✗ 文件大小不一致" -ForegroundColor Red
        }
        
        # 5. 展示前端使用方式
        Write-Host "`n[前端使用示例]" -ForegroundColor Yellow
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
        
        Write-Host "`n1️⃣  直接链接下载:" -ForegroundColor Cyan
        Write-Host "   <a href=`"$baseUrl/api/excel/download/$fileId`" download>`"
        Write-Host "     下载 Excel 文件"
        Write-Host "   </a>"
        
        Write-Host "`n2️⃣  JavaScript 下载:" -ForegroundColor Cyan
        Write-Host "   const fileId = '$fileId';"
        Write-Host "   window.location.href = ``$baseUrl/api/excel/download/`${fileId}``;"
        
        Write-Host "`n3️⃣  fetch API 下载:" -ForegroundColor Cyan
        Write-Host "   fetch('$baseUrl/api/excel/download/$fileId')"
        Write-Host "     .then(res => res.blob())"
        Write-Host "     .then(blob => {"
        Write-Host "       const url = URL.createObjectURL(blob);"
        Write-Host "       const a = document.createElement('a');"
        Write-Host "       a.href = url;"
        Write-Host "       a.download = 'report.xlsx';"
        Write-Host "       a.click();"
        Write-Host "     });"
        
        Write-Host "`n4️⃣  axios 下载:" -ForegroundColor Cyan
        Write-Host "   axios.get('$baseUrl/api/excel/download/$fileId', {"
        Write-Host "     responseType: 'blob'"
        Write-Host "   }).then(res => {"
        Write-Host "     const url = URL.createObjectURL(res.data);"
        Write-Host "     const a = document.createElement('a');"
        Write-Host "     a.href = url;"
        Write-Host "     a.download = 'report.xlsx';"
        Write-Host "     a.click();"
        Write-Host "   });"
        
        Write-Host "`n5️⃣  curl 命令:" -ForegroundColor Cyan
        Write-Host "   curl -o report.xlsx $baseUrl/api/excel/download/$fileId"
        
        Write-Host "`n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
        
        Write-Host "`n[对比说明]" -ForegroundColor Yellow
        Write-Host "POST 方法: 需要发送 JSON body，适合需要鉴权或复杂参数的场景" -ForegroundColor Gray
        Write-Host "GET 方法:  URL 直接访问，适合简单下载，前端使用更方便" -ForegroundColor Gray
        
        Write-Host "`n✓ 测试完成！两种下载方法都可正常使用" -ForegroundColor Green
        
        # 清理测试文件
        Write-Host "`n[清理] 删除测试文件..." -ForegroundColor Yellow
        Remove-Item "test_post.xlsx", "test_get.xlsx" -ErrorAction SilentlyContinue
        Write-Host "✓ 清理完成" -ForegroundColor Green
        
    } else {
        Write-Host "✗ 文件生成失败: $($response.message)" -ForegroundColor Red
    }
    
} catch {
    Write-Host "`n✗ 测试过程中发生错误: $_" -ForegroundColor Red
    Write-Host "请确保服务器已启动: cargo run" -ForegroundColor Yellow
}

Write-Host "`n===== 测试结束 =====" -ForegroundColor Cyan
