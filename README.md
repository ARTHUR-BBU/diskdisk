# DiskDisk 🧹

**A safe and efficient disk cleanup tool for Windows**

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows-blue.svg)](https://www.microsoft.com/windows)

[English](README.md) | [简体中文](README.zh-CN.md)

---

## 📖 Project Overview

DiskDisk is a modern disk cleanup tool written in Rust that supports **38 cache types**, covering popular domestic and international applications. It helps users free up disk space and improve system performance through intelligent scanning and safe cleanup.

### Key Features

- 🔍 **Smart Scanning** - Automatically identify system and application caches
- 🛡️ **Safe & Reliable** - Whitelist mechanism, only clean known safe caches
- 🚀 **High Performance** - Rust-powered, extremely fast scanning and cleanup
- 💻 **Dual Interface** - Both CLI command-line and GUI graphical interface
- 📊 **Detailed Statistics** - Display cache size, file count, and more

### Supported Cache Types (38 types)

#### Windows System (8 types)
- Windows temp files, prefetch, logs, update cache
- Defender scan history, explorer cache, font cache, IE cache

#### Browsers (6 types)
- Chrome, Edge, Firefox
- Doubao, QQ Browser, 360 Browser

#### Development Tools (5 types)
- NPM, Cargo, Pip, Docker, Node GYP

#### Office Software (4 types)
- VS Code, JetBrains IDE, Microsoft Office, WPS Office

#### Social & Communication (3 types)
- WeChat, QQ, DingTalk

#### Video Editing (1 type)
- JianyingPro

#### Note-taking & Docs (1 type)
- Notion

#### Video Conferencing (1 type)
- Zoom

#### Security Software (2 types)
- 360 Security, 360 Browser (separate from security software)

#### SDK & Tools (1 type)
- SecretSDK

#### Trading Platforms (1 type)
- FTNN Exchange

#### OCR Tools (1 type)
- 2345OCR

#### Python Environment (1 type)
- Python Environment

#### Others (4 types)
- Intel Graphics cache, Baidu Netdisk, Recycle Bin

---

## 📊 Project Progress

### Completed Features ✅

#### Core Library (diskdisk-core)
- ✅ 38 cache type definitions and configurations
- ✅ Automatic environment variable expansion (%APPDATA%, %LOCALAPPDATA%, etc.)
- ✅ Cache scanner
- ✅ Cache cleaner
- ✅ Error handling and logging
- ✅ Unit tests (7 test cases)

#### CLI Interface
- ✅ Command-line argument parsing
- ✅ Scan mode (`--scan`)
- ✅ Clean mode (`--clean`)
- ✅ Verbose output (`--verbose`)
- ✅ User confirmation mechanism
- ✅ Cleanup statistics report

#### GUI Interface (Tauri)
- ✅ Project structure setup
- ✅ Tauri 2.0 integration
- ✅ Modern UI (Tailwind CSS)
- ✅ Scan page (real-time progress)
- ✅ Clean page (selective cleanup)
- ✅ Settings page (configuration options)
- ✅ Frontend-backend communication (Tauri commands)

#### Documentation
- ✅ README.md (project homepage)
- ✅ QUICKSTART.md (quick start guide)
- ✅ USAGE.md (complete guide)
- ✅ System scan report
- ✅ Domestic app scan report
- ✅ GUI development summary

### Real Performance 🎉

```
Scan results: 93,293 files, 9.07 GB (pre-expansion: 30 cache types)
Post-expansion scan: 46,536 files, 5.44 GB (38 cache types)
Actual cleanup: 25,248 files, 2.55 GB (pre-expansion)
New caches cleanable: ~4.86 GB
```

### Pending Features 📝

#### High Priority
- [ ] Add application icons
- [ ] Improve error handling
- [ ] Cache detail viewer
- [ ] Cleanup history

#### Medium Priority
- [ ] Scheduled cleanup
- [ ] Custom cache paths
- [ ] Portable version

#### Low Priority
- [ ] Auto-update
- [ ] Cloud sync for settings
- [ ] Plugin system

---

## 🚀 Quick Start

### Prerequisites

- Windows 10/11
- [Rust toolchain](https://www.rust-lang.org/tools/install) (for building from source)

### Installation

#### Option 1: Build from Source

```bash
# Clone repository
git clone https://github.com/ARTHUR-BBU/diskdisk.git
cd diskdisk

# Build project
cargo build --release

# CLI executable
.\target\release\diskdisk.exe
```

#### Option 2: Download Pre-built Binary (Coming Soon)

Pre-built binaries will be available in [Releases](https://github.com/ARTHUR-BBU/diskdisk/releases) page.

### Basic Usage

#### CLI - Scan Mode

```bash
diskdisk.exe --scan
```

**Output:**
```
Scanning for cache files...
Found the following caches:
  • Notion - 1.88 GB (16070 files)
  • Zoom - 571.58 MB (970 files)
  • 360 Security - 572.45 MB (5235 files)
  ...

Total: 46,536 files, 5.44 GB
```

#### CLI - Clean Mode

```bash
diskdisk.exe --clean
```

**Interactive confirmation:**
```
⚠️  Cleanup mode will delete files!
About to delete: 46,536 files, 5.44 GB

Continue? [y/N]: y

✨ Cleanup complete!
  Files deleted: 46,536
  Space freed: 5.44 GB
```

#### GUI - Desktop Application

```bash
cd desktop-gui
cargo tauri dev
```

---

## 🏗️ Architecture

### Workspace Structure

```
diskdisk/
├── crates/
│   ├── core/           # Core library (platform-agnostic)
│   │   ├── scanner.rs       # Cache scanning logic
│   │   ├── cleaner.rs       # Cache cleanup logic
│   │   └── cache_types.rs   # Cache type definitions
│   └── cli/            # Command-line interface
├── desktop-gui/        # Tauri GUI application
├── docs/               # Project documentation
├── Cargo.toml          # Workspace configuration
└── README.md           # This file
```

### Design Principles

1. **Core Library Agnostic** - `diskdisk-core` doesn't depend on any UI framework
2. **Safety First** - All deletions require explicit confirmation
3. **Extensibility** - Easy to add new cache types via `CacheType` and `CacheLocation`
4. **Cross-platform Potential** - Currently focused on Windows, but architecture considers future cross-platform support

---

## 🛠️ Development Guide

### Build Project

```bash
# Build all components
cargo build

# Build only core library
cargo build -p diskdisk-core

# Build only CLI
cargo build -p diskdisk

# Build GUI (requires Tauri dependencies)
cargo build -p diskdisk-gui
```

### Run Tests

```bash
# Run all tests
cargo test

# Run tests for core library only
cargo test -p diskdisk-core

# Run with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Check code style
cargo fmt --check

# Run linter
cargo clippy -- -D warnings
```

---

## 📚 Documentation

- [QUICKSTART.md](QUICKSTART.md) - 3-minute quick start guide
- [USAGE.md](USAGE.md) - Complete usage instructions
- [CHANGELOG.md](CHANGELOG.md) - Version changelog
- [docs/system-scan-report.md](docs/system-scan-report.md) - System scan report
- [docs/domestic-apps-scan-report.md](docs/domestic-apps-scan-report.md) - Domestic apps report
- [docs/cache-expansion-v0.2.0.md](docs/cache-expansion-v0.2.0.md) - v0.2.0 expansion details

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Adding New Cache Types

1. Add new variant to `CacheType` enum in `crates/core/src/cache_types.rs`
2. Add cache location configuration in `crates/core/src/scanner.rs`
3. Update `parse_cache_type()` in `desktop-gui/src/lib.rs` (for GUI)
4. Test with `cargo run --bin diskdisk -- --scan`

Example:
```rust
// cache_types.rs
pub enum CacheType {
    // ... existing types
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

## 📄 License

Dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

---

## 🙏 Acknowledgments

- [Rust](https://www.rust-lang.org/) - The systems programming language
- [Tauri](https://tauri.app/) - Cross-platform desktop framework
- [walkdir](https://github.com/BurntSushi/walkdir) - Directory traversal
- All contributors and users of DiskDisk

---

## 📮 Contact & Support

- **Issues**: [GitHub Issues](https://github.com/ARTHUR-BBU/diskdisk/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ARTHUR-BBU/diskdisk/discussions)
- **Author**: ARTHUR-BBU

---

Made with ❤️ and Rust by [ARTHUR-BBU](https://github.com/ARTHUR-BBU)
