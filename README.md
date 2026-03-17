# DiskDisk 🧹

**一个安全、高效的 Windows 磁盘清理工具**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows-blue.svg)](https://www.microsoft.com/windows)

---

## 📖 项目简介

DiskDisk 是一个用 Rust 编写的现��化磁盘清理工具，支持 **30 种缓存类型**，覆盖国内外主流应用程序。通过智能扫描和安全清理，帮助用户释放磁盘空间，提升系统性能。

### 核心特性

- 🔍 **智能扫描** - 自动识别系统和应用程序缓存
- 🛡️ **安全可靠** - 白名单机制，只清理已知安全的缓存
- 🚀 **高性能** - Rust 实现，扫描和清理速度极快
- 💻 **双界面** - 提供 CLI 命令行和 GUI 图形界面
- 📊 **详细统计** - 显示缓存大小、文件数量等详细信息

### 支持的缓存类型（30 种）

#### Windows 系统（8 种）
- Windows 临时文件、预读取、日志、更新缓存
- Defender 扫描历史、资源管理器缓存、字体缓存、IE 缓存

#### 浏览器（6 种）
- Chrome、Edge、Firefox
- 豆包浏览器��QQ浏览器、360浏览器

#### 开发工具（5 种）
- NPM、Cargo、Pip、Docker、Node GYP

#### 办公软件（4 种）
- VS Code、JetBrains IDE、Microsoft Office、WPS Office

#### 社交通信（3 种）
- 微信、QQ、钉钉

#### 其他（4 种）
- 剪映专业版、Intel 图形缓存、百度网盘、回收站

---

## 📊 项目进度

### 已完成功能 ✅

#### 核心库（diskdisk-core）
- ✅ 30 种缓存类型定义和配置
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
- �� 现代化 UI（Tailwind CSS）
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
扫描结果: 93,293 个文件, 9.07 GB
实际清理: 25,248 个文件, 2.55 GB
```

### 待完善功能 📝

#### 高优先级
- [ ] 添加应用图标
- [ ] 完善错误处理
- [ ] 缓存详情查看
- [ ] 清理历史记录

#### 中优先级
- [ ] 定时清理功能
- [ ] 白名单/黑名单
- [ ] 桌面通知
- [ ] 深色模式

#### 低优先级
- [ ] 多语言支持
- [ ] 自动更新
- [ ] 系统托盘
- [ ] 数据导出

---

## 🏗️ 项目架构

### Workspace 结构

```
diskdisk/
├── crates/
│   ├── core/              # 核心库（platform-agnostic）
│   │   ├── src/
│   │   │   ├── lib.rs              # 库入口
│   │   │   ├── cache_types.rs      # 30 种缓存类型定义
│   │   │   ├── scanner.rs          # 缓存扫描器
│   │   │   ├── cleaner.rs          # 缓存清理器
│   │   │   └── utils/
│   │   │       ├── mod.rs
│   │   │       └── expand_env.rs   # 环境变量展开
│   │   └── Cargo.toml
│   └── cli/               # 命令行界面
│       ├── src/
│       │   └── main.rs              # CLI 入口
│       └── Cargo.toml
├── desktop-gui/          # 图形界面（Tauri）
│   ├── src/
│   │   ├── lib.rs                  # Tauri commands
│   │   └── main.rs                 # GUI 入口
│   ├── ui/
│   │   └── index.html              # 单页应用
│   ├── icons/
│   ├── Cargo.toml
│   └── tauri.conf.json
├── docs/                # 项目文档
│   ├── system-scan-report.md
│   ├── domestic-apps-scan-report.md
│   └── gui-development-summary.md
├── Cargo.toml           # Workspace 配置
├── README.md            # 本文件
├── QUICKSTART.md        # 快速启动
└── USAGE.md             # 使用指南
```

### 核心设计原则

1. **核心库无关界面** - `diskdisk-core` 不依赖任何 UI 框架
2. **安全第一** - 所有删除操作需明确确认，支持 `--dry-run` 预览模式
3. **可扩展性** - 通过 `CacheType` 和 `CacheLocation` 轻松添加新的缓存类型
4. **跨平台潜力** - 虽然当前专注 Windows，但架构设计考虑了未来跨平台支持

---

## 🔧 环境配置

### 系统要求

- **操作系统**: Windows 10/11（推荐）
- **内存**: 2 GB RAM+
- **磁盘空间**: 100 MB（用于编译）
- **权限**: 管理员权限（清理系统缓存时需要）

### 开发环境

#### 必需工具

1. **Rust** (1.70+)
   ```bash
   # Windows: 下载并运行 rustup-init.exe
   # https://rustup.rs/

   # Linux/macOS:
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Cargo**（随 Rust 自动安装）

#### 可选工具

3. **Node.js** (18+) - GUI 开发
4. **Tauri CLI** - GUI 构建
   ```bash
   cargo install tauri-cli --version "^2.0"
   ```

### 依赖说明

#### 核心库依赖
- `thiserror` - 错误处理
- `tracing` - 日志框架
- `serde` - 序列化/反序列化
- `walkdir` - 文件系统遍历

#### GUI 依赖
- `tauri` (2.0) - 应用框架
- `tauri-plugin-shell` - Shell 插件
- `tauri-plugin-dialog` - 对话框插件
- `tauri-plugin-fs` - 文件系统插件

---

## 🚀 使用方法

### CLI 命令行版本

#### 快速开始

```bash
# 1. 克隆项目
git clone https://github.com/yourusername/diskdisk.git
cd diskdisk

# 2. 构建项目
cargo build --release

# 3. 运行扫描
cargo run --bin diskdisk -- --scan

# 4. 运行清理
cargo run --bin diskdisk -- --clean
```

#### 命令说明

```bash
# 扫描模式（默认）
cargo run --bin diskdisk -- --scan

# 清理模式（需确认）
cargo run --bin diskdisk -- --clean

# 详细输出
cargo run --bin diskdisk -- --verbose

# 查看帮助
cargo run --bin diskdisk -- --help
```

#### 输出示例

```
🧹 DiskDisk - Windows 磁盘清理工具

正在扫描缓存文件...

发现以下缓存：

  • Windows 临时文件 - 629.28 KB (9 文件)
  • Chrome 浏览器缓存 - 213.89 MB (1243 文件)
  • WPS Office 缓存 - 6.39 GB (66322 文件)
  • 微信缓存 - 1.03 MB (33 文件)

总计: 93293 个文件, 9.07 GB
```

### GUI 图形界面版本

#### 开发模式运行

```bash
# 1. 安装 Tauri CLI
cargo install tauri-cli --version "^2.0"

# 2. 启动开发服务器
cd desktop-gui
cargo tauri dev
```

#### 构建独立应用

```bash
# Windows
cargo tauri build

# 生成的安装包位于：
# desktop-gui/target/release/bundle/msi/
```

#### 界面功能

1. **扫描页面** - 显示所有缓存类型和大小
2. **清理页面** - 选择性清理，支持勾选/取消
3. **设置页面** - 配置清理选项和偏好

---

## 🔮 未来扩展方向

### 短期目标（1-3 个月）

#### 功能完善
- [ ] **定时清理** - 每周/每月自动清理
- [ ] **清理历史** - 记录清理历史和统计
- [ ] **白名单** - 用户自定义保留规则
- [ ] **通知系统** - 清理完成后的桌面通知
- [ ] **大文件检测** - 扫描并提示大型临时文件

#### 性能优化
- [ ] **并行扫描** - 使用多线程加速扫描
- [ ] **增量扫描** - 只扫描上次扫描后有变化的内容
- [ ] **缓存数据库** - 记录已扫描的文件，避免重复扫描

#### 用户体验
- [ ] **深色模式** - 界面主题切换
- [ ] **多语言** - 英文界面支持
- [ ] **进度估算** - 显示预估剩余时间

### 中期目标（3-6 个月）

#### 跨平台支持
- [ ] **Linux 支持** - 支持 Debian/Ubuntu/Fedora
- [ ] **macOS 支持** - 支持 Intel 和 Apple Silicon
- [ ] **自动平台检测** - 根据系统自动调整缓存路径

#### 高级功能
- [ ] **浏览器扩展** - 浏览器缓存实时清理
- [ ] **服务模式** - Windows 服务，后台自动清理
- [ ] **命令行管道** - 支持与其他工具集成
- [ ] **配置导入导出** - 在不同机器间共享配置

#### GUI 增强
- [ ] **拖拽清理** - 拖拽文件/文件夹到应用进行清理
- [ ] **磁盘分析** - 可视化磁盘空间使用情况
- [ ] **实时监控** - 监控缓存增长情况
- [ ] **清理计划** - 灵活的定时清理策略

### 长期目标（6-12 个月）

#### 企业级功能
- [ ] **团队版** - 支持企业部署和集中管理
- [ ] **API 接口** - 提供 REST API 供其他工具调用
- [ ] **插件系统** - 允许第三方扩展缓存类型
- [ ] **云同步** - 配置和白名单云端同步

#### 智能化
- [ ] **AI 驱动清理建议** - 智能推荐可安全清理的缓存
- [ ] **机器学习** - 学习用户的清理习惯
- [ ] **异常检测** - 识别异常大的缓存增长

---

## 🧪 开发指南

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test -p diskdisk-core

# 显示测试输出
cargo test -- --nocapture

# 运行文档测试
cargo test --doc
```

### 代码格式化

```bash
# 格式化代码
cargo fmt

# 检查格式
cargo fmt --check
```

### Lint 检查

```bash
# 运行 Clippy
cargo clippy

# 修复建议
cargo clippy --fix
```

### 添加新的缓存类型

1. **在 `crates/core/src/cache_types.rs` 中添加类型**:
   ```rust
   pub enum CacheType {
       // ... 现有类型
       MyAppCache,  // 新类型
   }
   ```

2. **在 `scanner.rs` 中配置路径**:
   ```rust
   CacheLocation {
       cache_type: CacheType::MyAppCache,
       paths: vec![r"%LOCALAPPDATA%\MyApp\Cache".to_string()],
       description: "My Application Cache",
       dangerous: false,
   }
   ```

---

## 🤝 贡献指南

欢迎贡献！请遵循以下步骤：

1. **Fork 本仓库**
2. **创建特性分支** (`git checkout -b feature/AmazingFeature`)
3. **提交更改** (`git commit -m 'Add some AmazingFeature'`)
4. **推送到分支** (`git push origin feature/AmazingFeature`)
5. **开启 Pull Request**

### 代码规范

- 遵循 Rust 官方代码风格
- 使用 `cargo fmt` 格式化代码
- 通过 `cargo clippy` 检查
- 添加必要的单元测试
- 更新相关文档

---

## 📄 许可证

本项目采用双重许可：

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

你可以任选其一使用。

---

## 🙏 致谢

- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Tailwind CSS](https://tailwindcss.com/) - CSS 框架
- [walkdir](https://github.com/BurntSushi/walkdir) - 文件系统遍历
- [clap](https://github.com/clap-rs/clap) - 命令行参数解析

---

## 📮 联系方式

- **问题反馈**: [GitHub Issues](https://github.com/yourusername/diskdisk/issues)
- **功能建议**: [GitHub Discussions](https://github.com/yourusername/diskdisk/discussions)
- **Email**: your-email@example.com

---

## 🌟 项目亮点

### 技术亮点

- ✅ **现代化架构** - 模块化设计，核心库独立于界面
- ✅ **内存安全** - Rust 的所有权系统防止内存错误
- ✅ **高性能** - 零成本抽象，快速扫描和清理
- ✅ **类型安全** - 编译时检查，减少运行时错误
- ✅ **可维护性** - 清晰的代码结构，完善的文档

### 实测效果

- ✅ **扫描速度** - 1-3 分钟（30 种缓存类型）
- ✅ **清理速度** - 10-30 秒（25,000+ 文件）
- ✅ **空间释放** - 实测清理 2.55 GB
- ✅ **安全性** - 无误删除，所有操作经过确认

---

## ⚡ 快速链接

- **[快速启动](QUICKSTART.md)** - 3 分钟上手
- **[完整指南](USAGE.md)** - 详细使用说明
- **[GUI 开发](desktop-gui/README.md)** - GUI 开发文档
- **[系统扫描报告](docs/system-scan-report.md)** - 国际应用缓存
- **[国内应用报告](docs/domestic-apps-scan-report.md)** - 国内应用缓存

---

**如果这个项目对你有帮助，请给个 ⭐ Star！**

---

*最后更新：2026-03-17 | 版本：v0.1.0*
