# 添加 cargo 到 PATH
$cargoPath = Join-Path $env:USERPROFILE ".cargo\bin"
$env:PATH = "$cargoPath;$env:PATH"

# 设置 Visual Studio 环境
$vsPath = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"
$env:VSCMD_START_DIR = Get-Location

cmd /c "`"`"$vsPath`" && set`"" | ForEach-Object {
    if ($_ -match '^(.+?)=(.*)$') {
        [Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
    }
}

# 运行 cargo build
& "$cargoPath\cargo.exe" build $args
