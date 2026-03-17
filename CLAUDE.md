# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

**DiskDisk** 是一个 Windows 磁盘清理工具，用于安全地清理系统和应用程序产生的缓存文件。

- **核心语言**: Rust
- **架构**: 模块化设计，核心库独立于界面
- **目标平台**: Windows 10/11

## 项目架构

项目采用 Workspace 架构，分为以下模块：

```
diskdisk/
├── crates/
│   ├── core/          # 核心库（platform-agnostic）
│   │   ├── scanner.rs      # 缓存扫描逻辑
│   │   ├── cleaner.rs      # 缓存清理逻辑
│   │   └── cache_types.rs  # 缓存类型定义
│   └── cli/           # 命令行界面
├── desktop-gui/       # Tauri GUI 应用（未来）
└── docs/              # 项目文档
```

### 核心设计原则

1. **核心库无关界面**: `diskdisk-core` 不依赖任何 UI 框架，可被 CLI 和 GUI 共用
2. **安全第一**: 所有删除操作需明确确认，支持 `--dry-run` 预览模式
3. **可扩展性**: 通过 `CacheType` 和 `CacheLocation` 轻松添加新的缓存类型
4. **跨平台潜力**: 虽然当前专注 Windows，但架构设计考虑了未来跨平台支持

## 开发命令

### 构建项目
```bash
# 构建所有组件
cargo build

# 仅构建核心库
cargo build -p diskdisk-core

# 仅构建 CLI
cargo build -p diskdisk
```

### 运行和测试
```bash
# 运行 CLI（扫描模式）
cargo run --bin diskdisk -- scan

# 运行 CLI（详细输出）
cargo run --bin diskdisk -- scan --verbose

# 运行测试
cargo test

# 运行格式化检查
cargo fmt --check

# 运行 Clippy 检查
cargo clippy -- -D warnings
```

### 依赖管理
```bash
# 更新依赖
cargo update

# 添加新依赖（示例：添加到 core）
cargo add -p diskdisk-core <crate-name>

# 检查依赖树
cargo tree
```

## 关键技术决策

### 为什么选择 Rust？

- **内存安全**: 文件系统操作涉及大量指针处理，Rust 的所有权模型防止内存错误
- **零成本抽象**: 高级抽象不影响性能，对于扫描大量文件很重要
- **单一可执行文件**: 无需用户安装运行时，分发方便
- **Windows API 支持**: `windows` crate 提供完整的 Win32 API 绑定

### 缓存扫描策略

1. **路径白名单**: 只扫描已知的缓存目录，不遍历整个文件系统
2. **文件类型识别**: 部分缓存通过扩展名识别（如 `.pf`, `.log`）
3. **权限处理**: 优雅处理权限不足的情况，记录警告但继续扫描

### 安全机制

- `dangerous` 标记：标识可能影响系统稳定性的缓存类型
- 默认启用 dry-run 模式，用户显式指定才真正删除
- 清理前显示详细的文件统计
- 保留清理日志（待实现）

## 扩展新的缓存类型

添加新的缓存类型需要修改两个文件：

1. **`crates/core/src/cache_types.rs`**: 在 `CacheType` enum 中添加新类型
2. **`crates/core/src/scanner.rs`**: 在 `get_default_locations()` 中添加路径规则

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

## Windows 开发注意事项

- 环境变量扩展：使用 `std::env::var()` 或 `dirs` crate 处理 `%APPDATA%` 等变量
- 长路径支持：Windows 有 260 字符路径限制，考虑使用 UNC 路径前缀 `\\?\`
- 文件锁定：某些系统文件可能被占用，需要优雅处理 `std::io::ErrorKind::PermissionDenied`
- 管理员权限：部分系统目录需要提升权限才能访问

## 未来计划

- [ ] Tauri GUI 实现
- [ ] 定时任务支持
- [ ] 清理历史记录
- [ ] 自定义清理规则
- [ ] 自动更新功能
- [ ] 便携版（无需安装）
