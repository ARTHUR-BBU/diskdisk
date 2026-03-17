@echo off
REM DiskDisk GUI 开发脚本
REM 用于快速启动开发环境

echo ========================================
echo   DiskDisk GUI - 开发模式启动
echo ========================================
echo.

REM 检查 Rust 是否安装
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [错误] 未找到 Rust，请先安装 Rust:
    echo   https://www.rust-lang.org/tools/install
    pause
    exit /b 1
)

REM 检查 Node.js 是否安装
where node >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [警告] 未找到 Node.js，建议安装以获得最佳开发体验
    echo   https://nodejs.org/
    echo.
)

echo [信息] 检查依赖...
cargo check

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [错误] 依赖检查失败，请运行: cargo build
    pause
    exit /b 1
)

echo.
echo [信息] 启动 Tauri 开发服务器...
echo.
echo 提示:
echo   - 应用窗口将自动打开
echo   - 修改代码后会自动重新加载
echo   - 按 Ctrl+C 停止服务器
echo.

REM 启动开发服务器
cargo tauri dev

pause
