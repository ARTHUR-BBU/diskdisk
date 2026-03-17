use clap::Parser;
use diskdisk_core::{Scanner, Cleaner};
use tracing_subscriber;

/// DiskDisk - Windows 磁盘清理工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 扫描模式：不删除任何文件，仅显示可清理的内容
    #[arg(short, long)]
    scan: bool,

    /// 清理模式：实际删除缓存文件
    #[arg(short, long)]
    clean: bool,

    /// 指定要清理的缓存类型（可多次指定）
    #[arg(long, value_name = "TYPE")]
    cache_type: Vec<String>,

    /// 详细输出
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // 初始化日志
    if args.verbose {
        tracing_subscriber::fmt().init();
    }

    println!("🧹 DiskDisk - Windows 磁盘清理工具\n");

    let scanner = Scanner::new();

    if args.scan || (!args.clean && !args.scan) {
        // 扫描模式（默认）
        println!("正在扫描缓存文件...\n");

        let results = scanner.scan_all()?;

        if results.is_empty() {
            println!("✨ 没有发现可清理的缓存");
            return Ok(());
        }

        // 显示结果
        println!("发现以下缓存：\n");
        for result in &results {
            println!("  • {} - {} ({} 文件)", result.cache_type.display_name(), format_size(result.size), result.file_count);
        }

        let total_size: u64 = results.iter().map(|r| r.size).sum();
        let total_files: u64 = results.iter().map(|r| r.file_count).sum();

        println!("\n总计: {} 个文件, {}", total_files, format_size(total_size));
    }

    if args.clean {
        println!("⚠️  清理模式将删除文件！\n");

        // 先执行扫描
        let results = scanner.scan_all()?;

        if results.is_empty() {
            println!("✨ 没有发现可清理的缓存");
            return Ok(());
        }

        // 显示摘要
        let total_size: u64 = results.iter().map(|r| r.size).sum();
        let total_files: u64 = results.iter().map(|r| r.file_count).sum();

        println!("即将删除以下缓存：\n");
        for result in &results {
            println!("  • {} - {} ({} 文件)", result.cache_type.display_name(), format_size(result.size), result.file_count);
        }

        println!("\n总计: {} 个文件, {}\n", total_files, format_size(total_size));

        // 用户确认
        print!("是否继续? [y/N]: ");
        use std::io::Write;
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().to_lowercase().starts_with('y') {
            println!("已取消清理");
            return Ok(());
        }

        // 执行清理
        let cleaner = Cleaner::new(false);
        let stats = cleaner.clean(&results)?;

        // 显示结果
        println!("\n✨ 清理完成！");
        println!("  删除文件: {}", stats.files_deleted);
        println!("  释放空间: {}", format_size(stats.space_freed));
    }

    Ok(())
}

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
