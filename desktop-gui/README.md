# DiskDisk GUI 桌面应用

基于 Tauri 2.0 构建的现代化磁盘清理工具图形界面。

## 功能特性

### 🎨 现代化界面
- 简洁优雅的 UI 设计
- 紫色渐变主题
- 响应式布局
- 流畅的动画效果

### 🔍 智能扫描
- 实时进度显示
- 分类展示缓存
- 详细统计信息
- 危险缓存标记

### 🧹 安全清理
- 选择性清理
- 大缓存警告
- 清理前确认
- 详细的清理报告

### ⚙️ 灵活配置
- 多种清理选项
- 定时清理计划
- 白名单/黑名单
- 桌面通知

## 技术栈

### 后端 (Rust)
- **Tauri 2.0**: 轻量级桌面应用框架
- **diskdisk-core**: 核心清理逻辑
- **tokio**: 异步运行时
- **serde**: 序列化/反序列化

### 前端 (Web)
- **HTML5**: 页面结构
- **Tailwind CSS**: 样式框架
- **JavaScript ES6+**: 交互逻辑
- **Tauri API**: 前后端通信

## 开发指南

### 环境要求

#### 系统要求
- Windows 10/11 (推荐)
- macOS 10.15+
- Linux (主流发行版)

#### 必需工具
1. **Rust** (1.70+)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** (18+) - 用于前端开发
   ```bash
   # 从 https://nodejs.org 下载安装
   ```

3. **Tauri CLI**
   ```bash
   cargo install tauri-cli --version "^2.0"
   ```

### 构建步骤

#### 1. 克隆项目
```bash
git clone https://github.com/yourusername/diskdisk.git
cd diskdisk/desktop-gui
```

#### 2. 开发模式运行
```bash
# 方式 1: 使用 cargo tauri
cargo tauri dev

# 方式 2: 使用 Tauri CLI
tauri dev
```

这将：
- 启动前端开发服务器
- 编译 Rust 后端
- 打开应用窗口
- 支持热重载

#### 3. 构建发布版本
```bash
# 构建所有平台
cargo tauri build

# 仅构建当前平台
cargo tauri build --target universal-apple-darwin  # macOS
cargo tauri build --target x86_64-pc-windows-msvc # Windows
cargo tauri build --target x86_64-unknown-linux-gnu # Linux
```

构建产物位于 `src-tauri/target/release/bundle/`

### 开发工作流

#### 修改前端
编辑 `ui/index.html`，保存后会自动刷新

#### 修改后端
编辑 `src/lib.rs` 或 `src/main.rs`，Tauri 会自动重新编译

#### 添加新功能

1. **添加 Rust 命令** (src/lib.rs)
```rust
#[tauri::command]
async fn my_new_command(param: String) -> Result<String, String> {
    Ok(format!("Hello, {}!", param))
}

// 注册命令
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        get_cache_types,
        scan_all_caches,
        clean_selected_caches,
        my_new_command  // 添加这里
    ])
```

2. **调用命令** (ui/index.html)
```javascript
const result = await invoke('my_new_command', { param: 'World' });
console.log(result);
```

## 项目结构

```
desktop-gui/
├── src/                    # Rust 后端源码
│   ├── lib.rs             # Tauri 命令实现
│   └── main.rs            # 应用入口
├── ui/                     # 前端资源
│   └── index.html         # 主界面
├── icons/                  # 应用图标
│   └── README.md          # 图标说明
├── Cargo.toml             # Rust 依赖配置
├── tauri.conf.json        # Tauri 配置
├── build.rs               # 构建脚本
└── README.md              # 本文件
```

## 配置说明

### tauri.conf.json

主要配置项：

```json
{
  "app": {
    "windows": [{
      "width": 1200,        // 窗口宽度
      "height": 800,        // 窗口高度
      "minWidth": 900,      // 最小宽度
      "minHeight": 600      // 最小高度
    }]
  },
  "bundle": {
    "identifier": "com.diskdisk.app",  // 应用 ID
    "icon": ["icons/icon.png"]         // 应用图标
  }
}
```

## 常见问题

### Q: 如何修改应用图标？
A: 将图标文件放入 `icons/` 目录，参考 `icons/README.md`

### Q: 前端页面不刷新？
A: 尝试按 `Ctrl+R` (Windows) 或 `Cmd+R` (macOS) 刷新

### Q: 编译失败？
A: 确保安装了系统依赖：
- Windows: WebView2
- macOS: Xcode Command Line Tools
- Linux: 参考 https://tauri.app/v1/guides/getting-started/prerequisites

### Q: 如何调试？
A:
1. 后端日志: 在终端查看
2. 前端日志: 按 `F12` 打开开发者工具
3. 启用调试模式: 在 `tauri.conf.json` 中设置 `"debug": true`

## 性能优化

### 减小体积
- 启用 `lto = true` 在 Cargo.toml
- 使用 `--release` 模式构建
- 移除不必要的依赖

### 提升速度
- 使用异步操作
- 优化扫描算法
- 缓存扫描结果

## 打包分发

### Windows
```bash
cargo tauri build --target x86_64-pc-windows-msvc
# 生成: target/release/bundle/msi/
#       target/release/bundle/nsis/
```

### macOS
```bash
cargo tauri build --target universal-apple-darwin
# 生成: target/release/bundle/dmg/
#       target/release/bundle/macos/
```

### Linux
```bash
cargo tauri build --target x86_64-unknown-linux-gnu
# 生成: target/release/bundle/deb/
#       target/release/bundle/appimage/
```

## 许可证

MIT OR Apache-2.0

## 贡献

欢迎提交 Issue 和 Pull Request！

## 致谢

- [Tauri](https://tauri.app/) - 现代化桌面应用框架
- [Tailwind CSS](https://tailwindcss.com/) - 实用优先的 CSS 框架
- [diskdisk-core](../crates/core/) - 核心清理逻辑
