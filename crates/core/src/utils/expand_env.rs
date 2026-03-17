//! 环境变量展开工具

use std::env;

/// 展开 Windows 环境变量（%APPDATA%, %LOCALAPPDATA% 等）
///
/// # 示例
///
/// ```
/// use diskdisk_core::utils::expand_env_variables;
///
/// // 将 %APPDATA%\npm-cache 展开为实际路径
/// let expanded = expand_env_variables(r"%APPDATA%\npm-cache");
/// assert!(expanded.contains("npm-cache"));
/// ```
pub fn expand_env_variables(path: &str) -> String {
    let mut result = path.to_string();

    // 常用 Windows 环境变量
    let env_vars = vec![
        ("APPDATA", env::var("APPDATA").ok()),
        ("LOCALAPPDATA", env::var("LOCALAPPDATA").ok()),
        ("TEMP", env::var("TEMP").ok()),
        ("TMP", env::var("TMP").ok()),
        ("USERPROFILE", env::var("USERPROFILE").ok()),
        ("PROGRAMDATA", env::var("PROGRAMDATA").ok()),
        ("PROGRAMFILES", env::var("PROGRAMFILES").ok()),
        ("PROGRAMFILES(X86)", env::var("PROGRAMFILES(X86)").ok()),
    ];

    for (var_name, var_value) in env_vars {
        if let Some(value) = var_value {
            result = result.replace(&format!("%{}%", var_name), &value);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_env_variables() {
        env::set_var("TEST_VAR", "C:\\Test");
        assert_eq!(
            expand_env_variables("%TEST_VAR%\\file.txt"),
            "C:\\Test\\file.txt"
        );
        env::remove_var("TEST_VAR");
    }

    #[test]
    fn test_no_expand_needed() {
        assert_eq!(expand_env_variables("C:\\Windows\\Temp"), "C:\\Windows\\Temp");
    }

    #[test]
    fn test_multiple_env_vars() {
        env::set_var("DIR1", "C:\\Dir1");
        env::set_var("DIR2", "C:\\Dir2");
        assert_eq!(
            expand_env_variables("%DIR1%\\%DIR2%\\file.txt"),
            "C:\\Dir1\\C:\\Dir2\\file.txt"
        );
        env::remove_var("DIR1");
        env::remove_var("DIR2");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(expand_env_variables(""), "");
    }
}
