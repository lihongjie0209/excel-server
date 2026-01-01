# 测试文件持久化功能
Write-Host "===== 测试 Excel Server 文件持久化功能 =====" -ForegroundColor Cyan

$baseUrl = "http://localhost:3000"

# 1. 启动服务器（后台）
Write-Host "`n[步骤 1] 启动服务器..." -ForegroundColor Yellow
$serverProcess = Start-Process -FilePath "cargo" -ArgumentList "run" -PassThru -NoNewWindow
Start-Sleep -Seconds 5

try {
    # 2. 生成文件并获取 file_id
    Write-Host "`n[步骤 2] 生成 Excel 文件并获取 file_id..." -ForegroundColor Yellow
    $dsl = Get-Content "examples/simple.json" -Raw
    $response = Invoke-RestMethod -Uri "$baseUrl/api/excel/async" -Method Post -Body $dsl -ContentType "application/json"
    
    if ($response.success) {
        $fileId = $response.data.file_id
        Write-Host "✓ 文件生成成功，file_id: $fileId" -ForegroundColor Green
        
        # 3. 下载文件验证
        Write-Host "`n[步骤 3] 下载文件验证..." -ForegroundColor Yellow
        $downloadReq = @{ file_id = $fileId } | ConvertTo-Json
        $downloadResponse = Invoke-RestMethod -Uri "$baseUrl/api/excel/download" -Method Post -Body $downloadReq -ContentType "application/json"
        
        if ($downloadResponse.success) {
            Write-Host "✓ 文件下载成功" -ForegroundColor Green
        } else {
            Write-Host "✗ 文件下载失败: $($downloadResponse.message)" -ForegroundColor Red
        }
        
        # 4. 检查存储状态
        Write-Host "`n[步骤 4] 检查存储状态..." -ForegroundColor Yellow
        $statusResponse = Invoke-RestMethod -Uri "$baseUrl/api/excel/status" -Method Post
        Write-Host "✓ 当前存储文件数: $($statusResponse.data.file_count)" -ForegroundColor Green
        
        # 5. 停止服务器
        Write-Host "`n[步骤 5] 停止服务器..." -ForegroundColor Yellow
        Stop-Process -Id $serverProcess.Id -Force
        Start-Sleep -Seconds 2
        Write-Host "✓ 服务器已停止" -ForegroundColor Green
        
        # 6. 验证文件系统持久化
        Write-Host "`n[步骤 6] 验证文件系统持久化..." -ForegroundColor Yellow
        $tempDir = "./temp"
        if (Test-Path $tempDir) {
            $dataFiles = Get-ChildItem -Path $tempDir -Filter "*.dat"
            $metaFiles = Get-ChildItem -Path $tempDir -Filter "*.meta.json"
            Write-Host "✓ 数据文件数量: $($dataFiles.Count)" -ForegroundColor Green
            Write-Host "✓ 元数据文件数量: $($metaFiles.Count)" -ForegroundColor Green
            
            # 显示元数据内容
            if ($metaFiles.Count -gt 0) {
                $metaContent = Get-Content $metaFiles[0].FullName -Raw | ConvertFrom-Json
                Write-Host "`n元数据示例:" -ForegroundColor Cyan
                Write-Host "  file_id: $($metaContent.file_id)"
                Write-Host "  filename: $($metaContent.filename)"
                Write-Host "  created_timestamp: $($metaContent.created_timestamp)"
            }
        } else {
            Write-Host "✗ temp 目录不存在" -ForegroundColor Red
        }
        
        # 7. 重新启动服务器
        Write-Host "`n[步骤 7] 重新启动服务器..." -ForegroundColor Yellow
        $serverProcess = Start-Process -FilePath "cargo" -ArgumentList "run" -PassThru -NoNewWindow
        Start-Sleep -Seconds 5
        
        # 8. 验证文件是否自动加载
        Write-Host "`n[步骤 8] 验证文件自动加载..." -ForegroundColor Yellow
        $statusResponse = Invoke-RestMethod -Uri "$baseUrl/api/excel/status" -Method Post
        Write-Host "✓ 重启后文件数: $($statusResponse.data.file_count)" -ForegroundColor Green
        
        # 9. 尝试下载之前的文件
        Write-Host "`n[步骤 9] 下载重启前的文件..." -ForegroundColor Yellow
        $downloadReq = @{ file_id = $fileId } | ConvertTo-Json
        $downloadResponse = Invoke-RestMethod -Uri "$baseUrl/api/excel/download" -Method Post -Body $downloadReq -ContentType "application/json"
        
        if ($downloadResponse.success) {
            Write-Host "✓ 持久化验证成功！文件在重启后仍可访问" -ForegroundColor Green
        } else {
            Write-Host "✗ 持久化验证失败: $($downloadResponse.message)" -ForegroundColor Red
        }
        
    } else {
        Write-Host "✗ 文件生成失败: $($response.message)" -ForegroundColor Red
    }
    
} catch {
    Write-Host "`n✗ 测试过程中发生错误: $_" -ForegroundColor Red
} finally {
    # 清理：停止服务器
    Write-Host "`n[清理] 停止服务器..." -ForegroundColor Yellow
    if ($serverProcess -and !$serverProcess.HasExited) {
        Stop-Process -Id $serverProcess.Id -Force
    }
    Write-Host "✓ 测试完成" -ForegroundColor Green
}

Write-Host "`n===== 测试结束 =====" -ForegroundColor Cyan
