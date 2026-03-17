//! 缓存类型定义和扫描规则

use serde::{Deserialize, Serialize};

/// 缓存类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CacheType {
    // Windows 系统
    WindowsTemp,
    WindowsPrefetch,
    WindowsLogs,
    WindowsUpdateCache,
    WindowsDefenderCache,
    WindowsExplorerCache,
    WindowsFontCache,
    WindowsINetCache,

    // 浏览器
    BrowserChrome,
    BrowserEdge,
    BrowserFirefox,
    BrowserDoubao,      // 豆包浏览器
    BrowserQQBrowser,   // QQ浏览器
    Browser360,         // 360浏览器

    // 开发工具
    NpmCache,
    CargoCache,
    PipCache,
    DockerCache,
    NodeGypCache,

    // 办公软件
    VsCodeCache,
    JetBrainsCache,
    OfficeCache,
    WPSOfficeCache,     // WPS Office

    // 社交通信
    WeChatCache,        // 微信
    QQCache,            // QQ
    DingTalkCache,      // 钉钉

    // 视频编辑
    JianyingProCache,   // 剪映专业版

    // 硬件/图形
    IntelGraphicsCache,

    // 云存储
    BaiduNetdiskCache,  // 百度网盘

    // 笔记和文档
    NotionCache,        // Notion 笔记

    // 视频会议
    ZoomCache,         // Zoom 视频会议

    // 安全软件
    Sec360Cache,        // 360 安全软件
    Browser360Browser, // 360浏览器（区分于安全软件）

    // SDK 和工具
    SecretSDKCache,    // secretsdk

    // 交易平台
    FTNNEfpCache,      // FTNN 交易所

    // OCR 工具
    OCR2345Cache,      // 2345OCR

    // Python 环境
    PythonEnvCache,     // Python 环境

    // 通用
    RecycleBin,
}

impl CacheType {
    pub fn display_name(&self) -> &'static str {
        match self {
            CacheType::WindowsTemp => "Windows 临时文件",
            CacheType::WindowsPrefetch => "Windows 预读取文件",
            CacheType::WindowsLogs => "Windows 日志",
            CacheType::WindowsUpdateCache => "Windows 更新缓存",
            CacheType::WindowsDefenderCache => "Windows Defender 扫描历史",
            CacheType::WindowsExplorerCache => "Windows 资源管理器缓存",
            CacheType::WindowsFontCache => "Windows 字体缓存",
            CacheType::WindowsINetCache => "Internet Explorer 缓存",
            CacheType::BrowserChrome => "Chrome 浏览器缓存",
            CacheType::BrowserEdge => "Edge 浏览器缓存",
            CacheType::BrowserFirefox => "Firefox 浏览器缓存",
            CacheType::BrowserDoubao => "豆包浏览器缓存",
            CacheType::BrowserQQBrowser => "QQ浏览器缓存",
            CacheType::Browser360 => "360浏览器缓存",
            CacheType::NpmCache => "NPM 缓存",
            CacheType::CargoCache => "Cargo 缓存",
            CacheType::PipCache => "Python Pip 缓存",
            CacheType::DockerCache => "Docker 缓存",
            CacheType::NodeGypCache => "Node.js GYP 缓存",
            CacheType::VsCodeCache => "VS Code 缓存",
            CacheType::JetBrainsCache => "JetBrains IDE 缓存",
            CacheType::OfficeCache => "Microsoft Office 缓存",
            CacheType::WPSOfficeCache => "WPS Office 缓存",
            CacheType::WeChatCache => "微信缓存",
            CacheType::QQCache => "QQ 缓存",
            CacheType::DingTalkCache => "钉钉缓存",
            CacheType::JianyingProCache => "剪映专业版缓存",
            CacheType::IntelGraphicsCache => "Intel 图形缓存",
            CacheType::BaiduNetdiskCache => "百度网盘缓存",
            CacheType::NotionCache => "Notion 笔记",
            CacheType::ZoomCache => "Zoom 视频会议",
            CacheType::Sec360Cache => "360 安全软件",
            CacheType::Browser360Browser => "360浏览器",
            CacheType::SecretSDKCache => "SecretSDK 缓存",
            CacheType::FTNNEfpCache => "FTNN 交易所缓存",
            CacheType::OCR2345Cache => "2345OCR 缓存",
            CacheType::PythonEnvCache => "Python 环境缓存",
            CacheType::RecycleBin => "回收站",
        }
    }
}

/// 缓存位置定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheLocation {
    pub cache_type: CacheType,
    pub paths: Vec<String>,
    pub description: &'static str,
    pub dangerous: bool, // 标记是否可能影响系统稳定性
}
