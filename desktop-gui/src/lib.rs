// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use diskdisk_core::{CacheType, Scanner, Cleaner};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

// 应用状态
struct AppState {
    scanner: Mutex<Scanner>,
    scan_results: Mutex<Vec<ScanResultItem>>,
}

// 扫描结果项
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScanResultItem {
    cache_type: String,
    display_name: String,
    path: String,
    size: u64,
    file_count: u64,
    dangerous: bool,
    selected: bool,
}

// 扫描进度
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScanProgress {
    current: usize,
    total: usize,
    message: String,
}

// 清理统计
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CleanStats {
    files_found: u64,
    total_size: u64,
    files_deleted: u64,
    space_freed: u64,
}

// 获取所有缓存类型
#[tauri::command]
async fn get_cache_types() -> Result<Vec<CacheTypeInfo>, String> {
    let scanner = Scanner::new();
    let locations = scanner.cache_locations;

    let cache_types: Vec<CacheTypeInfo> = locations
        .iter()
        .map(|loc| CacheTypeInfo {
            cache_type: format!("{:?}", loc.cache_type),
            display_name: loc.cache_type.display_name().to_string(),
            description: loc.description.to_string(),
            dangerous: loc.dangerous,
        })
        .collect();

    Ok(cache_types)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheTypeInfo {
    cache_type: String,
    display_name: String,
    description: String,
    dangerous: bool,
}

// 扫描所有缓存
#[tauri::command]
async fn scan_all_caches(
    state: State<'_, AppState>,
    window: tauri::Window,
) -> Result<Vec<ScanResultItem>, String> {
    let scanner = state.scanner.lock().unwrap();
    let locations = scanner.cache_locations.clone();
    drop(scanner);

    let mut results = Vec::new();
    let total = locations.len();

    for (index, location) in locations.iter().enumerate() {
        // 发送进度更新
        let progress = ScanProgress {
            current: index + 1,
            total,
            message: format!("正在扫描: {}", location.description),
        };

        window.emit("scan-progress", &progress).map_err(|e| e.to_string())?;

        // 扫描每个路径
        for path_str in &location.paths {
            use diskdisk_core::utils::expand_env_variables;
            use std::path::Path;

            let expanded_path = expand_env_variables(path_str);
            let path = Path::new(&expanded_path);

            if path.exists() {
                if let Ok(result) = scan_single_path(&expanded_path, location.cache_type) {
                    if result.size > 0 {
                        results.push(ScanResultItem {
                            cache_type: format!("{:?}", location.cache_type),
                            display_name: location.cache_type.display_name().to_string(),
                            path: result.path,
                            size: result.size,
                            file_count: result.file_count,
                            dangerous: location.dangerous,
                            selected: !location.dangerous, // 危险缓存默认不选中
                        });
                    }
                }
            }
        }
    }

    // 保存结果到状态
    *state.scan_results.lock().unwrap() = results.clone();

    // 发送完成事件
    window.emit("scan-complete", &results).map_err(|e| e.to_string())?;

    Ok(results)
}

// 扫描单个路径
fn scan_single_path(path: &str, cache_type: CacheType) -> Result<diskdisk_core::scanner::ScanResult, Box<dyn std::error::Error>> {
    use walkdir::WalkDir;

    let path_obj = std::path::Path::new(path);
    if !path_obj.exists() {
        return Err("Path not found".into());
    }

    let mut total_size = 0u64;
    let mut file_count = 0u64;

    for entry in WalkDir::new(path_obj).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                total_size += metadata.len();
                file_count += 1;
            }
        }
    }

    Ok(diskdisk_core::scanner::ScanResult {
        cache_type,
        path: path.to_string(),
        size: total_size,
        file_count,
    })
}

// 清理选中的缓存
#[tauri::command]
async fn clean_selected_caches(
    state: State<'_, AppState>,
    selected_items: Vec<ScanResultItem>,
    window: tauri::Window,
) -> Result<CleanStats, String> {
    // 显示确认对话框
    let total_size: u64 = selected_items.iter().map(|r| r.size).sum();
    let total_files: u64 = selected_items.iter().map(|r| r.file_count).sum();

    let confirm_message = format!(
        "即将删除 {} 个文件，释放 {} 空间。\n\n是否继续？",
        total_files,
        format_size(total_size)
    );

    window.emit("clean-confirm", &confirm_message).map_err(|e| e.to_string())?;

    // 执行清理
    let cleaner = Cleaner::new(false);
    let scan_results: Vec<diskdisk_core::scanner::ScanResult> = selected_items
        .iter()
        .map(|item| diskdisk_core::scanner::ScanResult {
            cache_type: parse_cache_type(&item.cache_type),
            path: item.path.clone(),
            size: item.size,
            file_count: item.file_count,
        })
        .collect();

    let stats = cleaner.clean(&scan_results).map_err(|e| e.to_string())?;

    let clean_stats = CleanStats {
        files_found: stats.files_found,
        total_size: stats.total_size,
        files_deleted: stats.files_deleted,
        space_freed: stats.space_freed,
    };

    // 发送完成事件
    window.emit("clean-complete", &clean_stats).map_err(|e| e.to_string())?;

    Ok(clean_stats)
}

// 解析缓存类型
fn parse_cache_type(type_str: &str) -> CacheType {
    match type_str {
        // Windows 系统
        "WindowsTemp" => CacheType::WindowsTemp,
        "WindowsPrefetch" => CacheType::WindowsPrefetch,
        "WindowsLogs" => CacheType::WindowsLogs,
        "WindowsUpdateCache" => CacheType::WindowsUpdateCache,
        "WindowsDefenderCache" => CacheType::WindowsDefenderCache,
        "WindowsExplorerCache" => CacheType::WindowsExplorerCache,
        "WindowsFontCache" => CacheType::WindowsFontCache,
        "WindowsINetCache" => CacheType::WindowsINetCache,
        // 浏览器
        "BrowserChrome" => CacheType::BrowserChrome,
        "BrowserEdge" => CacheType::BrowserEdge,
        "BrowserFirefox" => CacheType::BrowserFirefox,
        "BrowserDoubao" => CacheType::BrowserDoubao,
        "BrowserQQBrowser" => CacheType::BrowserQQBrowser,
        "Browser360" => CacheType::Browser360,
        // 开发工具
        "NpmCache" => CacheType::NpmCache,
        "CargoCache" => CacheType::CargoCache,
        "PipCache" => CacheType::PipCache,
        "DockerCache" => CacheType::DockerCache,
        "NodeGypCache" => CacheType::NodeGypCache,
        // 办公软件
        "VsCodeCache" => CacheType::VsCodeCache,
        "JetBrainsCache" => CacheType::JetBrainsCache,
        "OfficeCache" => CacheType::OfficeCache,
        "WPSOfficeCache" => CacheType::WPSOfficeCache,
        // 社交通信
        "WeChatCache" => CacheType::WeChatCache,
        "QQCache" => CacheType::QQCache,
        "DingTalkCache" => CacheType::DingTalkCache,
        // 视频编辑
        "JianyingProCache" => CacheType::JianyingProCache,
        // 硬件/图形
        "IntelGraphicsCache" => CacheType::IntelGraphicsCache,
        // 云存储
        "BaiduNetdiskCache" => CacheType::BaiduNetdiskCache,
        // 通用
        "RecycleBin" => CacheType::RecycleBin,
        _ => CacheType::WindowsTemp, // 默认值
    }
}

// 格式化文件大小
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState {
        scanner: Mutex::new(Scanner::new()),
        scan_results: Mutex::new(Vec::new()),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_cache_types,
            scan_all_caches,
            clean_selected_caches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
