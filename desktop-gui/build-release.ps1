# DiskDisk GUI 发布版构建脚本
# 用于构建可分发的安装包

param(
    [string]$Target = "",
    [switch]$Debug = $false
)

$ErrorActionPreference = "Stop"

function Write-Section {
    param([string]$Message)
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  $Message" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host ""
}

function Write-Step {
    param([string]$Message)
    Write-Host "[步骤] $Message" -ForegroundColor Green
}

function Write-Info {
    param([string]$Message)
    Write-Host "[信息] $Message" -ForegroundColor Gray
}

function Write-Success {
    param([string]$Message)
    Write-Host "[成功] $Message" -ForegroundColor Green
}

function Write-Error {
    param([string]$Message)
    Write-Host "[错误] $Message" -ForegroundColor Red
}

# 开始
Write-Section "DiskDisk GUI - 发布版构建"

# 检查 Rust
Write-Step "检查 Rust 安装..."
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "未找到 Rust，请先安装: https://www.rust-lang.org/tools/install"
    exit 1
}
Write-Success "Rust 已安装"

# 检查 Tauri CLI
Write-Step "检查 Tauri CLI..."
if (!(cargo tauri --version 2>$null)) {
    Write-Info "未找到 Tauri CLI，正在安装..."
    cargo install tauri-cli --version "^2.0"
}
Write-Success "Tauri CLI 已就绪"

# 检查图标
Write-Step "检查应用图标..."
$iconFiles = @(
    "icons\32x32.png",
    "icons\128x128.png",
    "icons\128x128@2x.png",
    "icons\icon.icns",
    "icons\icon.ico"
)

$missingIcons = $iconFiles | Where-Object { !(Test-Path $_) }
if ($missingIcons.Count -gt 0) {
    Write-Info "缺少图标文件: $($missingIcons -join ', ')"
    Write-Info "将使用 Tauri 默认图标（开发模式）"
    Write-Info "正式发布前请参考 icons\README.md 添加图标"
}

# 构建参数
$buildArgs = @("tauri", "build")

if ($Target) {
    $buildArgs += "--target"
    $buildArgs += $Target
}

if ($Debug) {
    $buildArgs += "--debug"
}

# 开始构建
Write-Section "开始构建"
Write-Info "构建参数: $($buildArgs -join ' ')"
Write-Host ""

Push-Location $PSScriptRoot
try {
    & cargo @buildArgs
    if ($LASTEXITCODE -eq 0) {
        Write-Success "构建完成！"

        $outputDir = if ($Debug) { "debug" } else { "release" }
        Write-Section "构建产物位置"
        Write-Info "安装包位于: src-tauri\target\$outputDir\bundle\"
    }
    else {
        Write-Error "构建失败"
        exit 1
    }
}
finally {
    Pop-Location
}

Write-Section "构建完成"
Write-Host ""
Write-Host "下一步:" -ForegroundColor Cyan
Write-Host "  1. 测试安装包"
Write-Host "  2. 在虚拟机中验证功能"
Write-Host "  3. 发布到 GitHub Releases"
Write-Host ""
