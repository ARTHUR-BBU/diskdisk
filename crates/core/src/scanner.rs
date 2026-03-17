//! 缓存扫描器

use crate::cache_types::CacheLocation;
use std::path::Path;

/// 扫描结果
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub cache_type: crate::cache_types::CacheType,
    pub path: String,
    pub size: u64,
    pub file_count: u64,
}

/// 缓存扫描器
pub struct Scanner {
    cache_locations: Vec<CacheLocation>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            cache_locations: Self::get_default_locations(),
        }
    }

    /// 获取默认的缓存位置（Windows）
    #[cfg(target_os = "windows")]
    fn get_default_locations() -> Vec<CacheLocation> {
        use crate::cache_types::CacheType;
        vec![
            // Windows 系统
            CacheLocation {
                cache_type: CacheType::WindowsTemp,
                paths: vec![
                    r"C:\Windows\Temp".to_string(),
                    r"%TEMP%".to_string(),
                ],
                description: "Windows 临时文件",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::WindowsPrefetch,
                paths: vec![r"C:\Windows\Prefetch".to_string()],
                description: "Windows 预读取文件",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::WindowsLogs,
                paths: vec![r"C:\Windows\Logs".to_string()],
                description: "Windows 日志",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::WindowsUpdateCache,
                paths: vec![r"C:\Windows\SoftwareDistribution\Download".to_string()],
                description: "Windows 更新缓存",
                dangerous: true,
            },
            CacheLocation {
                cache_type: CacheType::WindowsDefenderCache,
                paths: vec![
                    r"%PROGRAMDATA%\Microsoft\Windows Defender\Scans\History".to_string(),
                ],
                description: "Windows Defender 扫描历史",
                dangerous: true,
            },
            CacheLocation {
                cache_type: CacheType::WindowsExplorerCache,
                paths: vec![
                    r"%LOCALAPPDATA%\Microsoft\Windows\Explorer".to_string(),
                ],
                description: "Windows 资源管理器缓存（缩略图、图标）",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::WindowsFontCache,
                paths: vec![r"%LOCALAPPDATA%\Microsoft\FontCache".to_string()],
                description: "Windows 字体缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::WindowsINetCache,
                paths: vec![r"%LOCALAPPDATA%\Microsoft\Windows\INetCache".to_string()],
                description: "Internet Explorer 缓存",
                dangerous: false,
            },
            // 浏览器
            CacheLocation {
                cache_type: CacheType::BrowserChrome,
                paths: vec![
                    r"%LOCALAPPDATA%\Google\Chrome\User Data\Default\Cache".to_string(),
                    r"%LOCALAPPDATA%\Google\Chrome\User Data\Default\Code Cache".to_string(),
                    r"%LOCALAPPDATA%\Google\Chrome\User Data\ShaderCache".to_string(),
                    r"%LOCALAPPDATA%\Google\Chrome\User Data\GraphiteDawnCache".to_string(),
                ],
                description: "Chrome 浏览器缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::BrowserEdge,
                paths: vec![
                    r"%LOCALAPPDATA%\Microsoft\Edge\User Data\Default\Cache".to_string(),
                    r"%LOCALAPPDATA%\Microsoft\Edge\User Data\Default\Code Cache".to_string(),
                    r"%LOCALAPPDATA%\Microsoft\Edge\User Data\ShaderCache".to_string(),
                    r"%LOCALAPPDATA%\Microsoft\Edge\User Data\GraphiteDawnCache".to_string(),
                ],
                description: "Edge 浏览器缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::BrowserFirefox,
                paths: vec![r"%LOCALAPPDATA%\Mozilla\Firefox\Profiles\*\cache2".to_string()],
                description: "Firefox 浏览器缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::BrowserDoubao,
                paths: vec![
                    r"%LOCALAPPDATA%\Doubao\User Data\Default\Cache".to_string(),
                    r"%LOCALAPPDATA%\Doubao\User Data\Default\Code Cache".to_string(),
                    r"%LOCALAPPDATA%\Doubao\User Data\Default\GPUCache".to_string(),
                    r"%LOCALAPPDATA%\Doubao\User Data\Default\DawnCache".to_string(),
                ],
                description: "豆包浏览器缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::BrowserQQBrowser,
                paths: vec![r"%LOCALAPPDATA%\Tencent\QQBrowser\User Data\Default\Cache".to_string()],
                description: "QQ浏览器缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::Browser360,
                paths: vec![r"%APPDATA%\360browser\cache".to_string()],
                description: "360浏览器缓存",
                dangerous: false,
            },
            // 开发工具
            CacheLocation {
                cache_type: CacheType::NpmCache,
                paths: vec![r"%APPDATA%\npm-cache".to_string()],
                description: "NPM 缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::CargoCache,
                paths: vec![r"%USERPROFILE%\.cargo\registry\cache".to_string()],
                description: "Cargo 缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::PipCache,
                paths: vec![r"%LOCALAPPDATA%\pip\cache".to_string()],
                description: "Pip 缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::DockerCache,
                paths: vec![r"%LOCALAPPDATA%\Docker\cache".to_string()],
                description: "Docker 缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::NodeGypCache,
                paths: vec![r"%LOCALAPPDATA%\node-gyp\Cache".to_string()],
                description: "Node.js GYP 缓存",
                dangerous: false,
            },
            // 办公软件
            CacheLocation {
                cache_type: CacheType::VsCodeCache,
                paths: vec![
                    r"%APPDATA%\Code\Cache".to_string(),
                    r"%APPDATA%\Code\CachedData".to_string(),
                    r"%APPDATA%\Code\CachedConfigurations".to_string(),
                    r"%APPDATA%\Code\Code Cache".to_string(),
                    r"%APPDATA%\Code\DawnGraphiteCache".to_string(),
                    r"%APPDATA%\Code\DawnWebGPUCache".to_string(),
                ],
                description: "VS Code 缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::JetBrainsCache,
                paths: vec![r"%APPDATA%\JetBrains\*\*\caches".to_string()],
                description: "JetBrains IDE 缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::OfficeCache,
                paths: vec![
                    r"%LOCALAPPDATA%\Microsoft\Office\16.0\AddInClassifierCache".to_string(),
                    r"%LOCALAPPDATA%\Microsoft\Office\16.0\WebServiceCache".to_string(),
                    r"%LOCALAPPDATA%\Microsoft\Office\16.0\BackstageInAppNavCache".to_string(),
                ],
                description: "Microsoft Office 缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::WPSOfficeCache,
                paths: vec![
                    r"%LOCALAPPDATA%\Kingsoft\WPS Office\*".to_string(),
                    r"%APPDATA%\kingsoft\wps\addons".to_string(),
                    r"%APPDATA%\kingsoft\wps\download".to_string(),
                ],
                description: "WPS Office 缓存",
                dangerous: false,
            },
            // 社交通信
            CacheLocation {
                cache_type: CacheType::WeChatCache,
                paths: vec![
                    r"%APPDATA%\Tencent\WeChat\*\webnet".to_string(),
                    r"%APPDATA%\Tencent\WeChat\*\radium".to_string(),
                    r"%APPDATA%\Tencent\WeChat\wmpf".to_string(),
                    r"%APPDATA%\Tencent\WeChat\wmpf_app".to_string(),
                    r"%APPDATA%\Tencent\WeChat\xweb".to_string(),
                ],
                description: "微信缓存（小程序、网页等）",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::QQCache,
                paths: vec![
                    r"%APPDATA%\QQ\Cache".to_string(),
                    r"%APPDATA%\QQ\Code Cache".to_string(),
                ],
                description: "QQ 缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::DingTalkCache,
                paths: vec![
                    r"%APPDATA%\DingTalk\*\Sync_v2\cache".to_string(),
                    r"%APPDATA%\DingTalk\globalStorage\resource_cache".to_string(),
                    r"%APPDATA%\DingTalk\image_res_cache".to_string(),
                ],
                description: "钉钉缓存",
                dangerous: false,
            },
            // 视频编辑
            CacheLocation {
                cache_type: CacheType::JianyingProCache,
                paths: vec![
                    r"%LOCALAPPDATA%\JianyingPro\User Data\Cache".to_string(),
                    r"%LOCALAPPDATA%\Bytedance\JianyingPro\User Data\Cache".to_string(),
                ],
                description: "剪映专业版缓存",
                dangerous: false,
            },
            // 硬件/图形
            CacheLocation {
                cache_type: CacheType::IntelGraphicsCache,
                paths: vec![r"%LOCALAPPDATA%\Intel\ShaderCache".to_string()],
                description: "Intel 图形缓存",
                dangerous: false,
            },
            // 云存储
            CacheLocation {
                cache_type: CacheType::BaiduNetdiskCache,
                paths: vec![
                    r"%APPDATA%\baidu\BaiduNetdisk".to_string(),
                    r"%APPDATA%\BaiduYunGuanjia".to_string(),
                ],
                description: "百度网盘缓存",
                dangerous: false,
            },
            // 笔记和文档
            CacheLocation {
                cache_type: CacheType::NotionCache,
                paths: vec![
                    r"%APPDATA%\Notion".to_string(),
                ],
                description: "Notion 笔记缓存",
                dangerous: false,
            },
            // 视频会议
            CacheLocation {
                cache_type: CacheType::ZoomCache,
                paths: vec![
                    r"%APPDATA%\Zoom".to_string(),
                ],
                description: "Zoom 视频会议缓存",
                dangerous: false,
            },
            // 安全软件
            CacheLocation {
                cache_type: CacheType::Sec360Cache,
                paths: vec![
                    r"%APPDATA%\360Safe".to_string(),
                    r"%APPDATA%\360desktoplite".to_string(),
                    r"%APPDATA%\360Quarant".to_string(),
                ],
                description: "360 安全软件缓存",
                dangerous: false,
            },
            CacheLocation {
                cache_type: CacheType::Browser360Browser,
                paths: vec![
                    r"%APPDATA%\360browser".to_string(),
                ],
                description: "360浏览器缓存",
                dangerous: false,
            },
            // SDK 和工具
            CacheLocation {
                cache_type: CacheType::SecretSDKCache,
                paths: vec![
                    r"%APPDATA%\secoresdk".to_string(),
                ],
                description: "SecretSDK 缓存",
                dangerous: false,
            },
            // 交易平台
            CacheLocation {
                cache_type: CacheType::FTNNEfpCache,
                paths: vec![
                    r"%APPDATA%\FTNN".to_string(),
                ],
                description: "FTNN 交易所缓存",
                dangerous: false,
            },
            // OCR 工具
            CacheLocation {
                cache_type: CacheType::OCR2345Cache,
                paths: vec![
                    r"%APPDATA%\2345OCR".to_string(),
                ],
                description: "2345OCR 缓存",
                dangerous: false,
            },
            // Python 环境
            CacheLocation {
                cache_type: CacheType::PythonEnvCache,
                paths: vec![
                    r"%APPDATA%\Python".to_string(),
                    r"%APPDATA%\pip".to_string(),
                ],
                description: "Python 环境缓存",
                dangerous: false,
            },
        ]
    }

    #[cfg(not(target_os = "windows"))]
    fn get_default_locations() -> Vec<CacheLocation> {
        vec![]
    }

    /// 扫描所有缓存位置
    pub fn scan_all(&self) -> Result<Vec<ScanResult>, ScanError> {
        let mut results = Vec::new();

        for location in &self.cache_locations {
            for path_str in &location.paths {
                if let Ok(result) = self.scan_path(path_str, location.cache_type) {
                    results.push(result);
                }
            }
        }

        Ok(results)
    }

    /// 扫描单个路径
    fn scan_path(&self, path: &str, cache_type: crate::cache_types::CacheType) -> Result<ScanResult, ScanError> {
        use crate::utils::expand_env_variables;
        use walkdir::WalkDir;

        // 展开环境变量
        let expanded_path = expand_env_variables(path);
        let path = Path::new(&expanded_path);

        if !path.exists() {
            return Err(ScanError::PathNotFound(expanded_path));
        }

        let mut total_size = 0u64;
        let mut file_count = 0u64;

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                    file_count += 1;
                }
            }
        }

        Ok(ScanResult {
            cache_type,
            path: path.to_string_lossy().to_string(),
            size: total_size,
            file_count,
        })
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("路径不存在: {0}")]
    PathNotFound(String),

    #[error("权限不足: {0}")]
    PermissionDenied(String),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_creation() {
        let scanner = Scanner::new();
        assert!(!scanner.cache_locations.is_empty());
    }

    #[test]
    fn test_scanner_default() {
        let scanner = Scanner::default();
        assert!(!scanner.cache_locations.is_empty());
    }

    #[test]
    fn test_scan_all_returns_result() {
        let scanner = Scanner::new();
        // 即使没有缓存，也应该返回空结果而不是错误
        let result = scanner.scan_all();
        assert!(result.is_ok());
    }

    #[test]
    fn test_scan_nonexistent_path() {
        let scanner = Scanner::new();
        let result = scanner.scan_path("C:\\NonExistentPath12345", crate::cache_types::CacheType::WindowsTemp);
        assert!(result.is_err());
    }
}

