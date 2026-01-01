# Excel Server 文档快速启动脚本

Write-Host "===== Excel Server 文档系统 =====" -ForegroundColor Cyan

$docsDir = "documentation"

# 检查 Node.js
Write-Host "`n[检查] Node.js 环境..." -ForegroundColor Yellow
try {
    $nodeVersion = node --version
    Write-Host "✓ Node.js 版本: $nodeVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ 未安装 Node.js，请先安装: https://nodejs.org/" -ForegroundColor Red
    exit 1
}

# 进入文档目录
Set-Location $docsDir

# 检查是否已安装依赖
if (-not (Test-Path "node_modules")) {
    Write-Host "`n[安装] 正在安装依赖..." -ForegroundColor Yellow
    
    # 检测包管理器
    $packageManager = "npm"
    if (Get-Command "pnpm" -ErrorAction SilentlyContinue) {
        $packageManager = "pnpm"
    } elseif (Get-Command "yarn" -ErrorAction SilentlyContinue) {
        $packageManager = "yarn"
    }
    
    Write-Host "使用 $packageManager 安装依赖..." -ForegroundColor Gray
    
    switch ($packageManager) {
        "pnpm" { pnpm install }
        "yarn" { yarn install }
        default { npm install }
    }
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "✗ 依赖安装失败" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "✓ 依赖安装完成" -ForegroundColor Green
} else {
    Write-Host "`n✓ 依赖已安装" -ForegroundColor Green
}

# 提供选项
Write-Host "`n请选择操作:" -ForegroundColor Yellow
Write-Host "1. 启动开发服务器 (docs:dev)" -ForegroundColor Cyan
Write-Host "2. 构建静态文件 (docs:build)" -ForegroundColor Cyan
Write-Host "3. 预览构建结果 (docs:preview)" -ForegroundColor Cyan
Write-Host "4. 退出" -ForegroundColor Gray

$choice = Read-Host "`n请输入选项 (1-4)"

switch ($choice) {
    "1" {
        Write-Host "`n[启动] 开发服务器..." -ForegroundColor Yellow
        Write-Host "访问 http://localhost:5173 查看文档" -ForegroundColor Cyan
        npm run docs:dev
    }
    "2" {
        Write-Host "`n[构建] 生成静态文件..." -ForegroundColor Yellow
        npm run docs:build
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "`n✓ 构建完成！" -ForegroundColor Green
            Write-Host "静态文件位于: .vitepress/dist/" -ForegroundColor Cyan
            
            $size = (Get-ChildItem -Path ".vitepress/dist" -Recurse | Measure-Object -Property Length -Sum).Sum / 1MB
            Write-Host "总大小: $([math]::Round($size, 2)) MB" -ForegroundColor Gray
        } else {
            Write-Host "✗ 构建失败" -ForegroundColor Red
        }
    }
    "3" {
        Write-Host "`n[预览] 启动预览服务器..." -ForegroundColor Yellow
        
        if (-not (Test-Path ".vitepress/dist")) {
            Write-Host "✗ 未找到构建文件，请先运行构建" -ForegroundColor Red
            exit 1
        }
        
        Write-Host "访问 http://localhost:4173 预览构建结果" -ForegroundColor Cyan
        npm run docs:preview
    }
    "4" {
        Write-Host "`n再见！" -ForegroundColor Green
        exit 0
    }
    default {
        Write-Host "`n✗ 无效选项" -ForegroundColor Red
        exit 1
    }
}

Set-Location ..
