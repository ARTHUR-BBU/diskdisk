# DiskDisk 完整使用指南

**版本**: v1.0
**更新日期**: 2026-03-17
**支持**: CLI + GUI 双界面

---

## 📋 目录

1. [系统要求](#系统要求)
2. [安装 Rust](#安装-rust)
3. [CLI 命令行版本使用](#cli-命令行版本使用)
4. [GUI 图形界面版本使用](#gui-图形界面版本使用)
5. [生成应用图标](#生成应用图标)
6. [构建发布版本](#构建发布版本)
7. [常见问题](#常见问题)
8. [故障排除](#故障排除)

---

## 系统要求

### 最低要求
- **操作系统**: Windows 10/11（推荐）
- **内存**: 2 GB RAM
- **磁盘空间**: 100 MB（用于编译）
- **权限**: 管理员权限（清理系统缓存时需要）

### 开发环境
- **Rust**: 1.70 或更高版本
- **Node.js**: 18+ （GUI 开发，可选）
- **Git**: 用于克隆代码

---

## 安装 Rust

### Windows 安装

1. **下载 Rust 安装器**
   - 访问: https://www.rust-lang.org/tools/install
   - 下载 `rustup-init.exe`

2. **运行安装器**
   ```cmd
   rustup-init.exe
   ```

3. **选择默认选项**
   - 输入 `1` 并回车（继续安装）
   - 等待安装完成

4. **验证安装**
   ```cmd
   cargo --version
   rustc --version
   ```

5. **重启终端**
   - 关闭并重新打开命令提示符

### Linux/macOS 安装

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

## CLI 命令行版本使用

### 1. 进入项目目录

```bash
cd F:\diskdisk
```

### 2. 构建项目

```bash
cargo build --release
```

**首次构建可能需要 5-10 分钟**

### 3. 运行扫描

```bash
# 扫描所有缓存
cargo run --bin diskdisk -- scan

# 详细输出
cargo run --bin diskdisk -- scan --verbose

# 只扫描特定类型
cargo run --bin diskdisk -- scan --cache-type WindowsTemp --cache-type BrowserChrome
```

### 4. 运行清理

```bash
# 清理所有扫描到的缓存
cargo run --bin diskdisk -- clean

# 会显示确认提示
# 是否继续? [y/N]: y
```

### 5. 查看帮助

```bash
cargo run --bin diskdisk -- --help
```

---

## GUI 图形界面版本使用

### 方法 1: 开发模式（推荐用于测试）

#### 步骤 1: 安装 Tauri CLI

```bash
cargo install tauri-cli --version "^2.0"
```

**可能需要 5-10 分钟**

#### 步骤 2: 进入 GUI 目录

```bash
cd desktop-gui
```

#### 步骤 3: 启动开发服务器

**Windows:**
```bash
.\dev.bat
```

**或手动启动:**
```bash
cargo tauri dev
```

这将：
- ✅ 编译 Rust 后端
- ✅ 打开应用窗口
- ✅ 支持热重载（修改代码自动刷新）

#### 步骤 4: 使用界面

1. **扫描缓存**
   - 点击"开始扫描"按钮
   - 等待扫描完成
   - 查看扫描结果

2. **清理缓存**
   - 切换到"清理"标签页
   - 勾选要清理的缓存类型
   - 点击"开始清理"
   - 确认清理操作

3. **配置设置**
   - 切换到"设置"标签页
   - 配置清理选项
   - 设置定时清理

### 方法 2: 构建独立应用

#### 步骤 1: 生成应用图标

1. **打开图标生成器**
   ```bash
   cd desktop-gui/icons
   start generate-icons.html
   ```

2. **下载所有图标**
   - 点击"下载所有图标"按钮
   - 保存到 `desktop-gui/icons/` 目录

3. **转换为 ICO/ICNS**（可选）
   - **Windows ICO**: https://convertio.co/zh/png-ico/
   - **macOS ICNS**: https://cloudconvert.com/png-to-icns

#### 步骤 2: 构建应用

**Windows:**
```powershell
.\build-release.ps1
```

**或手动构建:**
```bash
cargo tauri build
```

#### 步骤 3: 查找构建产物

构建完成后，安装包位于：

**Windows:**
```
desktop-gui/target/release/bundle/msi/
desktop-gui/target/release/bundle/nsis/
```

**macOS:**
```
desktop-gui/target/release/bundle/dmg/
desktop-gui/target/release/bundle/macos/
```

**Linux:**
```
desktop-gui/target/release/bundle/deb/
desktop-gui/target/release/bundle/appimage/
```

#### 步骤 4: 安装和运行

**Windows:**
- 双击 `.msi` 或 `.exe` 安装包
- 按照安装向导完成安装
- 从开始菜单启动 DiskDisk

**macOS:**
- 打开 `.dmg` 文件
- 拖拽 DiskDisk 到应用程序文件夹
- 从 Launchpad 启动

**Linux:**
```bash
# Debian/Ubuntu
sudo dpkg -i diskdisk_0.1.0_amd64.deb

# AppImage（通用）
chmod +x DiskDisk_0.1.0_amd64.AppImage
./DiskDisk_0.1.0_amd64.AppImage
```

---

## 生成应用图标

### 使用内置图标生成器

1. **打开生成器页面**
   ```bash
   cd desktop-gui/icons
   start generate-icons.html
   ```

2. **下载图标**
   - 点击"下载所有图标"
   - 会下载 4 个 PNG 文件

3. **移动到正确位置**
   - 将下载的文件移动到 `desktop-gui/icons/` 目录

4. **转换格式**（发布时需要）
   - 使用在线工具将 PNG 转换为 ICO/ICNS
   - 将转换后的文件也放入 icons 目录

### 图标文件清单

```
desktop-gui/icons/
├── icon.svg              # 源文件（已提供）
├── icon-512.png          # 512x512 (你需要生成)
├── 128x128.png           # 128x128 (你需要生成)
├── 128x128@2x.png        # 256x256 (你需要生成)
├── 32x32.png             # 32x32  (你需要生成)
├── icon.ico              # Windows (你需要生成)
└── icon.icns             # macOS (你需要生成)
```

**注意**: 开发阶段可以使用默认图标，不影响功能测试。

---

## 构建发布版本

### 快速构建

```bash
cd desktop-gui

# Windows
.\build-release.ps1

# 或使用 cargo
cargo tauri build
```

### 高级构建选项

#### 指定目标平台

```bash
# Windows 32位
cargo tauri build --target i686-pc-windows-msvc

# Windows 64位
cargo tauri build --target x86_64-pc-windows-msvc

# macOS Intel
cargo tauri build --target x86_64-apple-darwin

# macOS Apple Silicon
cargo tauri build --target aarch64-apple-darwin

# macOS Universal (Intel + ARM)
cargo tauri build --target universal-apple-darwin

# Linux
cargo tauri build --target x86_64-unknown-linux-gnu
```

#### 调试构建

```bash
cargo tauri build --debug
```

调试构建速度更快，但文件体积更大。

---

## 常见问题

### Q1: 编译时出现 "error: linker `link.exe` not found"

**A**: 需要安装 Microsoft C++ Build Tools

1. 下载: https://visualstudio.microsoft.com/visual-cpp-build-tools/
2. 安装 "Desktop development with C++"
3. 重启终端

### Q2: GUI 窗口不显示

**A**: 检查 WebView2 是否安装

```bash
# 下载并安装 WebView2
# https://developer.microsoft.com/en-us/microsoft-edge/webview2/
```

### Q3: 扫描速度慢

**A**: 正常现象，首次扫描可能需要 1-3 分钟

### Q4: 清理时出现权限错误

**A**: 以管理员身份运行

```bash
# Windows
# 右键 -> 以管理员身份运行
```

### Q5: "找不到 cargo 命令"

**A**: 确保 Rust 已安装并在 PATH 中

```bash
# 重启终端或
# 手动添加到 PATH: %USERPROFILE%\.cargo\bin
```

---

## 故障排除

### 编译失败

1. **清理构建缓存**
   ```bash
   cargo clean
   ```

2. **更新依赖**
   ```bash
   cargo update
   ```

3. **重新构建**
   ```bash
   cargo build
   ```

### 运行时错误

1. **检查日志**
   - CLI: 查看终端输出
   - GUI: 按 `F12` 打开开发者工具

2. **启用详细日志**
   ```bash
   RUST_LOG=debug cargo run
   ```

### GUI 无法启动

1. **检查 Tauri 版本**
   ```bash
   cargo tauri --version
   ```

2. **重新安装 Tauri CLI**
   ```bash
   cargo install tauri-cli --force --version "^2.0"
   ```

3. **检查系统依赖**
   - Windows: WebView2 Runtime
   - Linux: libwebkit2gtk-4.0-dev
   - macOS: Xcode Command Line Tools

---

## 快速参考

### CLI 命令速查

```bash
# 构建项目
cargo build --release

# 扫描缓存
cargo run --bin diskdisk -- scan

# 清理缓存
cargo run --bin diskdisk -- clean

# 详细模式
cargo run --bin diskdisk -- scan --verbose
```

### GUI 命令速查

```bash
# 开发模式
cd desktop-gui
cargo tauri dev

# 构建应用
cargo tauri build

# 快速启动（Windows）
.\dev.bat
```

### 目录结构

```
diskdisk/
├── crates/
│   ├── core/           # 核心库
│   └── cli/            # CLI 界面
├── desktop-gui/         # GUI 界面
│   ├── src/            # Rust 后端
│   ├── ui/             # Web 前端
│   └── icons/          # 应用图标
├── docs/               # 文档
└── Cargo.toml          # 工作空间配置
```

---

## 下一步

### 学习更多

- 📖 [CLI 高级用法](./docs/advanced-cli.md)
- 🎨 [GUI 开发文档](./desktop-gui/README.md)
- 🔧 [缓存类型说明](./docs/system-scan-report.md)
- 🏠 [国内应用缓存](./docs/domestic-apps-scan-report.md)

### 反馈和贡献

- 🐛 [报告问题](https://github.com/yourusername/diskdisk/issues)
- 💡 [功能建议](https://github.com/yourusername/diskdisk/issues)
- 🤝 [贡献代码](https://github.com/yourusername/diskdisk/pulls)

---

**享受使用 DiskDisk！** 🧹✨
