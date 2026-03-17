//! DiskDisk Core - 磁盘清理核心库
//!
//! 提供跨平台的缓存扫描和清理功能

pub mod cleaner;
pub mod scanner;
pub mod cache_types;
pub mod utils;

pub use cleaner::Cleaner;
pub use scanner::Scanner;
pub use utils::expand_env_variables;

/// 清理结果统计
#[derive(Debug, Clone, serde::Serialize)]
pub struct CleanStats {
    /// 扫描到的文件数量
    pub files_found: u64,
    /// 总大小（字节）
    pub total_size: u64,
    /// 成功删除的文件数量
    pub files_deleted: u64,
    /// 释放的空间（字节）
    pub space_freed: u64,
}
