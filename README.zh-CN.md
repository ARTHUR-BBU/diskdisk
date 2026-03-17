# DiskDisk 🧹

**一个安全、高效的 Windows 磁盘清理工具**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows-blue.svg)](https://www.microsoft.com/windows)

[English](README.md) | [简体中文](README.zh-CN.md)

---

## 📖 项目简介

DiskDisk 是一个用 Rust 编写的现代化磁盘清理工具，支持 **38 种缓存类型**，覆盖国内外主流应用程序。通过智能扫描和安全清理，帮助用户释放磁盘空间，提升系统性能。

### 核心特性

- 🔍 **智能扫描** - 自动识别系统和应用程序缓存
- 🛡️ **安全可靠** - 白名单机制，只清理已知安全的缓存
- 🚀 **高性能** - Rust 实现，扫描和清理速度极快
- 💻 **双界面** - 提供 CLI 命令行和 GUI 图形界面
- 📊 **详细统计** - 显示缓存大小、文件数量等详细信息

### 支持的缓存类型（38 种）

#### Windows 系统（8 种）
- Windows 临时文件、预读取、日志、更新缓存
- Defender 扫描历史、资源管理器缓存、字体缓存、IE 缓存

#### 浏览器（6 种）
- Chrome、Edge、Firefox
- 豆包浏览器、QQ浏览器、360浏览器

#### 开发工具（5 种）
- NPM、Cargo、Pip、Docker、Node GYP

#### 办公软件（4 种）
- VS Code、JetBrains IDE、Microsoft Office、WPS Office

#### 社交通信（3 种）
- 微信、QQ、钉钉

#### 视频编辑（1 种）
- 剪映专业版

#### 笔记和文档（1 种）
- Notion 笔记

#### 视频会议（1 种）
- Zoom 视频会议

#### 安全软件（2 种）
- 360 安全软件、360浏览器（独立于安全软件）

#### SDK 和工具（1 种）
- SecretSDK 缓存

#### 交易平台（1 种）
- FTNN 交易所缓存

#### OCR 工具（1 种）
- 2345OCR 缓存

#### Python 环境（1 种）
- Python 环境缓存

#### 其他（4 种）
- Intel 图形缓存、百度网盘、回收站

---

## 📊 项目进度

### 已完成功能 ✅

#### 核心库（diskdisk-core）
- ✅ 38 种缓存类型定义和配置
- ✅ 环境变量自动展开（%APPDATA%, %LOCALAPPDATA% 等）
- ✅ 缓存扫描器（Scanner）
- ✅ 缓存清理器（Cleaner）
- ✅ 错误处理和日志记录
- ✅ 单元测试（7 个测试用例）

#### CLI 界面
- ✅ 命令行参数解析
- ✅ 扫描模式（`--scan`）
- ✅ 清理模式（`--clean`）
- ✅ 详细输出（`--verbose`）
- ✅ 用户确认机制
- ✅ 清理统计报告

#### GUI 界面（Tauri）
- ✅ 项目结构搭建
- ✅ Tauri 2.0 集成
- ✅ 现代化 UI（Tailwind CSS）
- ✅ 扫描页面（实时进度）
- ✅ 清理页面（选择性清理）
- ✅ 设置页面（配置选项）
- ✅ 前后端通信（Tauri commands）

#### 文档
- ✅ README.md（项目主页）
- ✅ QUICKSTART.md（快速启动）
- ✅ USAGE.md（完整指南）
- ✅ 系统扫描报告
- ✅ 国内应用扫描报告
- ✅ GUI 开发总结

### 实测效果 🎉

```
扫描结果: 93,293 个文件, 9.07 GB（扩展前：30 种缓存类型）
扩展后扫描结果: 46,536 个文件, 5.44 GB（38 种缓存类型）
实际清理: 25,248 个文件, 2.55 GB（扩展前）
新增缓存可清理: 约 4.86 GB
```

### 待完善功能 📝

#### 高优先级
- [ ] 添加应用图标
- [ ] 完善错误处理
- [ ] 缓存详情查看
- [ ] 清理历史记录

#### 中优先级
- [ ] 定时清理功能
- [ ] 自定义缓存路径
- [ ] 便携版

#### 低优先级
- [ ] 自动更新
- [ ] 云同步设置
- [ ] 插件系统

---

## 🚀 快速开始

### 环境要求

- Windows 10/11
- [Rust 工具链](https://www.rust-lang.org/zh-CN/tools/install)（从源码构建时需要）

### 安装

#### 方式 1：从源码构建

```bash
# 克隆仓库
git clone https://github.com/ARTHUR-BBU/diskdisk.git
cd diskdisk

# 构建项目
cargo build --release

# CLI 可执行文件
.\target\release\diskdisk.exe
```

#### 方式 2：下载预编译二进制（即将推出）

预编译版本将在 [Releases](https://github.com/ARTHUR-BBU/diskdisk/releases) 页面提供。

### 基本使用

#### CLI - 扫描模式

```bash
diskdisk.exe --scan
```

**输出示例：**
```
正在扫描缓存文件...
发现以下缓存：
  • Notion 笔记 - 1.88 GB (16070 文件)
  • Zoom 视频会议 - 571.58 MB (970 文件)
  • 360 安全软件 - 572.45 MB (5235 文件)
  ...

总计: 46,536 个文件, 5.44 GB
```

#### CLI - 清理模式

```bash
diskdisk.exe --clean
```

**交互式确认：**
```
⚠️  清理模式将删除文件！
即将删除: 46,536 个文件, 5.44 GB

是否继续? [y/N]: y

✨ 清理完成！
  删除文件: 46,536
  释放空间: 5.44 GB
```

#### GUI - 桌面应用

```bash
cd desktop-gui
cargo tauri dev
```

---

## 🏗️ 项目架构

### 工作空间结构

```
diskdisk/
├── crates/
│   ├── core/           # 核心库（platform-agnostic）
│   │   ├── scanner.rs       # 缓存扫描逻辑
│   │   ├── cleaner.rs       # 缓存清理逻辑
│   │   └── cache_types.rs   # 缓存类型定义
│   └── cli/            # 命令行界面
├── desktop-gui/        # Tauri GUI 应用
├── docs/               # 项目文档
├── Cargo.toml          # 工作空间配置
└── README.md           # 本文件
```

### 设计原则

1. **核心库无关界面** - `diskdisk-core` 不依赖任何 UI 框架
2. **安全第一** - 所有删除操作需明确确认
3. **可扩展性** - 通过 `CacheType` 和 `CacheLocation` 轻松添加新类型
4. **跨平台潜力** - 虽然当前专注 Windows，但架构考虑了未来跨平台支持

---

## 🛠️ 开发指南

### 构建项目

```bash
# 构建所有组件
cargo build

# 仅构建核心库
cargo build -p diskdisk-core

# 仅构建 CLI
cargo build -p diskdisk

# 构建 GUI（需要 Tauri 依赖）
cargo build -p diskdisk-gui
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 仅运行核心库测试
cargo test -p diskdisk-core

# 显示测试输出
cargo test -- --nocapture
```

### 代码质量

```bash
# 格式化代码
cargo fmt

# 检查代码格式
cargo fmt --check

# 运行 linter
cargo clippy -- -D warnings
```

---

## 📚 项目文档

- [QUICKSTART.md](QUICKSTART.md) - 3 分钟快速入门
- [USAGE.md](USAGE.md) - 完整使用指南
- [CHANGELOG.md](CHANGELOG.md) - 版本变更日志
- [docs/system-scan-report.md](docs/system-scan-report.md) - 系统扫描报告
- [docs/domestic-apps-scan-report.md](docs/domestic-apps-scan-report.md) - 国内应用报告
- [docs/cache-expansion-v0.2.0.md](docs/cache-expansion-v0.2.0.md) - v0.2.0 扩展详情

---

## 🤝 贡献指南

欢迎贡献！请随时提交 Pull Request。

### 添加新的缓存类型

1. 在 `crates/core/src/cache_types.rs` 的 `CacheType` 枚举中添加新变体
2. 在 `crates/core/src/scanner.rs` 中添加缓存位置配置
3. 更新 `desktop-gui/src/lib.rs` 中的 `parse_cache_type()`（用于 GUI）
4. 使用 `cargo run --bin diskdisk -- --scan` 测试

示例：
```rust
// cache_types.rs
pub enum CacheType {
    // ... 现有类型
    MyAppCache,
}

// scanner.rs
CacheLocation {
    cache_type: CacheType::MyAppCache,
    paths: vec![r"%LOCALAPPDATA%\MyApp\Cache".to_string()],
    description: "My Application Cache",
    dangerous: false,
}
```

---

## 📄 开源协议

双重许可：

- MIT License ([LICENSE-MIT](LICENSE-MIT) 或 http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) 或 http://www.apache.org/licenses/LICENSE-2.0)

你可以选择其中任一协议使用。

---

## 🙏 致谢

- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [walkdir](https://github.com/BurntSushi/walkdir) - 目录遍历库
- 所有 DiskDisk 的贡献者和用户

---

## 📮 联系与支持

- **问题反馈**: [GitHub Issues](https://github.com/ARTHUR-BBU/diskdisk/issues)
- **讨论交流**: [GitHub Discussions](https://github.com/ARTHUR-BBU/diskdisk/discussions)
- **作者**: ARTHUR-BBU

---

用 ❤️ 和 Rust 打造，by [ARTHUR-BBU](https://github.com/ARTHUR-BBU)
