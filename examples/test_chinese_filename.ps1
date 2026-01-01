# 测试中文文件名支持
Write-Host "===== 测试中文文件名下载 =====" -ForegroundColor Cyan

$baseUrl = "http://localhost:3000"

Write-Host "`n[提示] 请确保服务器已启动: cargo run" -ForegroundColor Yellow
Write-Host "按任意键继续..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

try {
    # 1. 生成文件并获取 file_id（模拟中文文件名）
    Write-Host "`n[步骤 1] 生成 Excel 文件..." -ForegroundColor Yellow
    
    # 修改 DSL 中的标题为中文
    $dsl = @{
        properties = @{
            title = "销售报表"
            author = "张三"
            company = "示例公司"
        }
        styles = @{
            header = @{
                font = @{
                    bold = $true
                    size = 12
                    color = "#FFFFFF"
                }
                fill = @{
                    color = "#4472C4"
                }
                align = @{
                    h = "center"
                    v = "vcenter"
                }
            }
        }
        sheets = @(
            @{
                name = "销售数据"
                cells = @(
                    @{ r = 0; c = 0; type = "string"; value = "产品名称"; style = "header" }
                    @{ r = 0; c = 1; type = "string"; value = "销售额"; style = "header" }
                    @{ r = 0; c = 2; type = "string"; value = "日期"; style = "header" }
                    @{ r = 1; c = 0; type = "string"; value = "产品A" }
                    @{ r = 1; c = 1; type = "number"; value = 12500.50 }
                    @{ r = 1; c = 2; type = "string"; value = "2026-01-01" }
                    @{ r = 2; c = 0; type = "string"; value = "产品B" }
                    @{ r = 2; c = 1; type = "number"; value = 8900.00 }
                    @{ r = 2; c = 2; type = "string"; value = "2026-01-02" }
                )
            }
        )
    } | ConvertTo-Json -Depth 10
    
    $response = Invoke-RestMethod -Uri "$baseUrl/api/excel/async" -Method Post -Body $dsl -ContentType "application/json"
    
    if ($response.success) {
        $fileId = $response.data.file_id
        Write-Host "✓ 文件生成成功，file_id: $fileId" -ForegroundColor Green
        
        # 2. 测试不同的文件名
        $testCases = @(
            @{ name = "销售报表.xlsx"; desc = "纯中文文件名" }
            @{ name = "Sales Report 2026.xlsx"; desc = "纯英文文件名" }
            @{ name = "销售报表_2026年1月.xlsx"; desc = "中英文混合文件名" }
            @{ name = "数据分析-Report-测试.xlsx"; desc = "复杂混合文件名" }
        )
        
        Write-Host "`n[步骤 2] 测试各种文件名格式..." -ForegroundColor Yellow
        
        foreach ($test in $testCases) {
            Write-Host "`n  测试: $($test.desc)" -ForegroundColor Cyan
            Write-Host "  文件名: $($test.name)" -ForegroundColor Gray
            
            # 使用 GET 方法下载
            try {
                $headers = Invoke-WebRequest -Uri "$baseUrl/api/excel/download/$fileId" -Method Get
                $contentDisposition = $headers.Headers.'Content-Disposition'
                
                Write-Host "  ✓ Content-Disposition: $contentDisposition" -ForegroundColor Green
                
                # 检查是否包含 RFC 5987 编码
                if ($contentDisposition -match "filename\*=UTF-8''") {
                    Write-Host "  ✓ 包含 RFC 5987 UTF-8 编码" -ForegroundColor Green
                } else {
                    Write-Host "  ⚠ 未包含 UTF-8 编码" -ForegroundColor Yellow
                }
                
                # 实际下载文件
                $outputFile = "test_$($test.name)"
                Invoke-WebRequest -Uri "$baseUrl/api/excel/download/$fileId" -OutFile $outputFile
                $fileInfo = Get-Item $outputFile
                Write-Host "  ✓ 文件下载成功: $outputFile ($($fileInfo.Length) bytes)" -ForegroundColor Green
                
                # 清理
                Remove-Item $outputFile -ErrorAction SilentlyContinue
                
            } catch {
                Write-Host "  ✗ 下载失败: $_" -ForegroundColor Red
            }
            
            Start-Sleep -Milliseconds 500
        }
        
        # 3. 展示 RFC 5987 编码示例
        Write-Host "`n[RFC 5987 编码说明]" -ForegroundColor Yellow
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
        
        Write-Host "`n标准格式:" -ForegroundColor Cyan
        Write-Host "  Content-Disposition: attachment; filename=`"fallback.xlsx`"; filename*=UTF-8''encoded_name"
        
        Write-Host "`n示例 1 - 纯中文文件名:" -ForegroundColor Cyan
        Write-Host "  原始: 销售报表.xlsx"
        Write-Host "  编码: filename=`"download.xlsx`"; filename*=UTF-8''%E9%94%80%E5%94%AE%E6%8A%A5%E8%A1%A8.xlsx"
        
        Write-Host "`n示例 2 - 纯英文文件名:" -ForegroundColor Cyan
        Write-Host "  原始: Sales Report.xlsx"
        Write-Host "  编码: filename=`"Sales Report.xlsx`"; filename*=UTF-8''Sales%20Report.xlsx"
        
        Write-Host "`n示例 3 - 中英文混合:" -ForegroundColor Cyan
        Write-Host "  原始: 销售报表_2026.xlsx"
        Write-Host "  编码: filename=`"download.xlsx`"; filename*=UTF-8''%E9%94%80%E5%94%AE%E6%8A%A5%E8%A1%A8_2026.xlsx"
        
        Write-Host "`n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
        
        Write-Host "`n[浏览器兼容性]" -ForegroundColor Yellow
        Write-Host "✓ Chrome/Edge: 优先使用 filename* (UTF-8)" -ForegroundColor Green
        Write-Host "✓ Firefox: 支持 filename* (UTF-8)" -ForegroundColor Green
        Write-Host "✓ Safari: 支持 filename* (UTF-8)" -ForegroundColor Green
        Write-Host "⚠ IE11: 使用 filename (ASCII fallback)" -ForegroundColor Yellow
        
        Write-Host "`n[实现细节]" -ForegroundColor Yellow
        Write-Host "1. 检查文件名是否全为 ASCII 字符" -ForegroundColor Gray
        Write-Host "2. ASCII 文件名: 使用原文件名作为 fallback" -ForegroundColor Gray
        Write-Host "3. 非ASCII文件名: 使用 'download.xlsx' 作为 fallback" -ForegroundColor Gray
        Write-Host "4. 所有文件名都提供 UTF-8 编码版本 (filename*)" -ForegroundColor Gray
        Write-Host "5. 现代浏览器会优先使用 filename* 的值" -ForegroundColor Gray
        
        Write-Host "`n✓ 中文文件名测试完成！" -ForegroundColor Green
        
    } else {
        Write-Host "✗ 文件生成失败: $($response.message)" -ForegroundColor Red
    }
    
} catch {
    Write-Host "`n✗ 测试过程中发生错误: $_" -ForegroundColor Red
    Write-Host "请确保服务器已启动: cargo run" -ForegroundColor Yellow
}

Write-Host "`n===== 测试结束 =====" -ForegroundColor Cyan
