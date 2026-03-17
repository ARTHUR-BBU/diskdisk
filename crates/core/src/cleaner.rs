//! 缓存清理器

use crate::scanner::ScanResult;
use crate::CleanStats;

/// 缓存清理器
pub struct Cleaner {
    dry_run: bool,
}

impl Cleaner {
    pub fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }

    /// 清理扫描结果中的所有缓存
    pub fn clean(&self, scan_results: &[ScanResult]) -> Result<CleanStats, CleanError> {
        let mut stats = CleanStats {
            files_found: scan_results.iter().map(|r| r.file_count).sum(),
            total_size: scan_results.iter().map(|r| r.size).sum(),
            files_deleted: 0,
            space_freed: 0,
        };

        for result in scan_results {
            match self.clean_path(&result.path) {
                Ok((deleted, freed)) => {
                    stats.files_deleted += deleted;
                    stats.space_freed += freed;
                }
                Err(e) => {
                    tracing::warn!("清理 {} 失败: {}", result.path, e);
                }
            }
        }

        Ok(stats)
    }

    /// 清理单个路径
    fn clean_path(&self, path: &str) -> Result<(u64, u64), CleanError> {
        use crate::utils::expand_env_variables;
        use std::fs;
        use std::path::Path;

        // 展开环境变量
        let expanded_path = expand_env_variables(path);
        let path = Path::new(&expanded_path);

        if self.dry_run {
            // 仅计算大小，不实际删除
            let (count, size) = self.calculate_size(path)?;
            return Ok((count, size));
        }

        // 先计算大小（用于统计）
        let (count, size) = self.calculate_size(path)?;

        // 实际删除
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else if path.is_file() {
            fs::remove_file(path)?;
        }

        Ok((count, size))
    }

    fn calculate_size(&self, path: &std::path::Path) -> Result<(u64, u64), CleanError> {
        use walkdir::WalkDir;

        let mut count = 0u64;
        let mut size = 0u64;

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Ok(metadata) = entry.metadata() {
                    size += metadata.len();
                    count += 1;
                }
            }
        }

        Ok((count, size))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CleanError {
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
    fn test_cleaner_creation() {
        let cleaner = Cleaner::new(true);
        assert!(cleaner.dry_run);
    }

    #[test]
    fn test_clean_dry_run() {
        let cleaner = Cleaner::new(true);
        let result = cleaner.clean(&[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_clean_with_empty_results() {
        let cleaner = Cleaner::new(false);
        let result = cleaner.clean(&[]);
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_eq!(stats.files_found, 0);
        assert_eq!(stats.files_deleted, 0);
    }
}

