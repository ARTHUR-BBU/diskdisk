# DiskDisk 快速启动指南

## 🚀 3 分钟快速开始

### 前提条件
- ✅ 已安装 Rust（如果没有，见下方安装步骤）

---

## 一、安装 Rust（5 分钟）

### Windows

```cmd
# 1. 下载并运行 rustup-init.exe
# https://rustup.rs/

# 2. 安装完成后，重启终端

# 3. 验证安装
cargo --version
```

### Linux/macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

## 二、选择使用方式

### 方式 1️⃣: 命令行版本（CLI）

**适合**: 高级用户、自动化脚本

```bash
cd F:\diskdisk

# 1. 构建（首次 5-10 分钟）
cargo build --release

# 2. 扫描缓存
cargo run --bin diskdisk -- scan

# 3. 清理缓存
cargo run --bin diskdisk -- clean
```

**就这么简单！** ✨

---

### 方式 2️⃣: 图形界面版本（GUI）

**适合**: 普通用户、想要可视化界面

#### 步骤 1: 安装 Tauri（2 分钟）

```bash
cargo install tauri-cli --version "^2.0"
```

#### 步骤 2: 启动开发模式

```bash
cd F:\diskdisk\desktop-gui

# Windows
.\dev.bat

# 或手动
cargo tauri dev
```

**就这么简单！** 🎨

应用窗口会自动打开，你将看到：
- 🔍 扫描按钮
- 🧹 清理选项
- ⚙️ 设置面板

---

## 📸 界面预览

### GUI 主界面

```
┌──────────────────────────────────────┐
│  🧹 DiskDisk - Windows 磁盘清理工具   │
├──────────────────────────────────────┤
│  [🔍 扫描] [🧹 清理] [⚙️ 设置]       │
│                                      │
│  ┌─ 统计 ─────────────────────────┐ │
│  │  💾 可清理: 3.2 GB              │ │
│  │  📁 文件: 15,234                │ │
│  │  🗂️ 类型: 30                    │ │
│  └─────────────────────────────────┘ │
│                                      │
│  ☑ Windows 系统 (1.2 GB)            │
│  ☑ 浏览器 (500 MB)                  │
│  ☑ 微信 (1.2 GB) ⚠️                 │
│                                      │
│  [开始清理]                          │
└──────────────────────────────────────┘
```

---

## 🎯 快速命令参考

### CLI 版本

```bash
# 扫描
cargo run --bin diskdisk -- scan

# 清理
cargo run --bin diskdisk -- clean

# 详细输出
cargo run --bin diskdisk -- scan --verbose

# 查看帮助
cargo run --bin diskdisk -- --help
```

### GUI 版本

```bash
cd desktop-gui

# 开发模式
cargo tauri dev

# 构建应用
cargo tauri build

# Windows 快速启动
.\dev.bat
```

---

## ⚡ 性能提示

1. **首次构建较慢**（5-10 分钟）- 正常现象
2. **后续构建很快**（< 1 分钟）
3. **扫描需要 1-3 ��钟** - 取决于系统
4. **清理很快**（通常 < 30 秒）

---

## 🛡️ 安全提示

- ✅ **默认安全**: 只清理已知缓存类型
- ✅ **清理前确认**: 需要你明确确认
- ✅ **危险缓存标记**: ⚠️ 标记的缓存需谨慎
- ✅ **可选择性**: 你可以选择清理哪些缓存

---

## 🆘 遇到问题？

### 编译错误
```bash
cargo clean
cargo build
```

### 找不到 cargo
- 重启终端
- 检查 Rust 是否正确安装

### GUI 窗口不显示
- Windows: 安装 WebView2 Runtime
- Linux: `sudo apt install libwebkit2gtk-4.0-dev`

---

## 📚 更多信息

- **完整文档**: [USAGE.md](./USAGE.md)
- **开发文档**: [desktop-gui/README.md](./desktop-gui/README.md)
- **扫描报告**: [docs/system-scan-report.md](./docs/system-scan-report.md)

---

**开始清理你的磁盘吧！** 🧹✨
