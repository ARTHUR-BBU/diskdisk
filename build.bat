@echo off
REM 设置 Visual Studio 环境
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"

REM 运行 cargo build
cargo build %*
